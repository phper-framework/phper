//! Apis relate to [crate::sys::zend_function_entry].

use std::{mem::zeroed, os::raw::c_char, sync::atomic::AtomicPtr};

use crate::{
    alloc::EBox,
    classes::ClassEntry,
    errors::ArgumentCountError,
    objects::Object,
    strings::ZendString,
    sys::*,
    utils::ensure_end_with_zero,
    values::{ExecuteData, SetVal, Val},
};
use std::{marker::PhantomData, mem::transmute, slice::from_raw_parts, str, str::Utf8Error};

pub(crate) trait Callable {
    fn call(&self, execute_data: &mut ExecuteData, arguments: &mut [Val], return_value: &mut Val);
}

pub(crate) struct Function<F, R>(pub(crate) F)
where
    F: Fn(&mut [Val]) -> R + Send + Sync,
    R: SetVal;

impl<F, R> Callable for Function<F, R>
where
    F: Fn(&mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    fn call(&self, _: &mut ExecuteData, arguments: &mut [Val], return_value: &mut Val) {
        let r = (self.0)(arguments);
        r.set_val(return_value);
    }
}

pub(crate) struct Method<F, R, T>
where
    F: Fn(&mut Object<T>, &mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    f: F,
    _p0: PhantomData<R>,
    _p1: PhantomData<T>,
}

impl<F, R, T> Method<F, R, T>
where
    F: Fn(&mut Object<T>, &mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    pub(crate) fn new(f: F) -> Self {
        Self {
            f,
            _p0: Default::default(),
            _p1: Default::default(),
        }
    }
}

impl<F, R, T> Callable for Method<F, R, T>
where
    F: Fn(&mut Object<T>, &mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    fn call(&self, execute_data: &mut ExecuteData, arguments: &mut [Val], return_value: &mut Val) {
        unsafe {
            let this = execute_data.get_this::<T>().unwrap();
            // TODO Fix the object type assertion.
            // assert!(this.get_type().is_object());
            let r = (self.f)(this, arguments);
            r.set_val(return_value);
        }
    }
}

#[repr(transparent)]
pub struct FunctionEntry {
    #[allow(dead_code)]
    inner: zend_function_entry,
}

pub struct FunctionEntity {
    pub(crate) name: String,
    pub(crate) handler: Box<dyn Callable>,
    pub(crate) arguments: Vec<Argument>,
}

impl FunctionEntity {
    pub(crate) fn new(
        name: impl ToString,
        handler: Box<dyn Callable>,
        arguments: Vec<Argument>,
    ) -> Self {
        let name = ensure_end_with_zero(name);
        FunctionEntity {
            name,
            handler,
            arguments,
        }
    }

    // Leak memory
    pub(crate) unsafe fn entry(&self) -> zend_function_entry {
        let mut infos = Vec::new();

        let require_arg_count = self.arguments.iter().filter(|arg| arg.required).count();
        infos.push(create_zend_arg_info(
            require_arg_count as *const c_char,
            false,
        ));

        for arg in &self.arguments {
            infos.push(create_zend_arg_info(
                arg.name.as_ptr().cast(),
                arg.pass_by_ref,
            ));
        }

        infos.push(zeroed::<zend_internal_arg_info>());

        let translator = CallableTranslator {
            callable: self.handler.as_ref(),
        };
        let mut last_arg_info: zend_internal_arg_info = translator.internal_arg_info;
        infos.push(last_arg_info);

        zend_function_entry {
            fname: self.name.as_ptr().cast(),
            handler: Some(invoke),
            arg_info: Box::into_raw(infos.into_boxed_slice()).cast(),
            num_args: self.arguments.len() as u32,
            flags: 0,
        }
    }
}

pub struct Argument {
    pub(crate) name: String,
    pub(crate) pass_by_ref: bool,
    pub(crate) required: bool,
}

impl Argument {
    pub fn by_val(name: impl ToString) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: false,
            required: true,
        }
    }

    pub fn by_ref(name: impl ToString) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: true,
            required: true,
        }
    }

    pub fn by_val_optional(name: impl ToString) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: false,
            required: false,
        }
    }

    pub fn by_ref_optional(name: impl ToString) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: true,
            required: false,
        }
    }
}

#[repr(transparent)]
pub struct ZendFunction {
    inner: zend_function,
}

impl ZendFunction {
    pub(crate) unsafe fn from_mut_ptr<'a>(ptr: *mut zend_function) -> &'a mut ZendFunction {
        let ptr = ptr as *mut Self;
        ptr.as_mut().expect("ptr shouldn't be null")
    }

    #[inline]
    pub fn as_ptr(&self) -> *const zend_function {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_function {
        &mut self.inner
    }

    pub fn get_name(&self) -> Result<String, Utf8Error> {
        unsafe {
            let s = phper_get_function_or_method_name(self.as_ptr());
            ZendString::from_raw(s).to_string()
        }
    }
}

/// Just for type transmutation.
pub(crate) union CallableTranslator {
    pub(crate) callable: *const dyn Callable,
    pub(crate) internal_arg_info: zend_internal_arg_info,
    pub(crate) arg_info: zend_arg_info,
}

/// The entry for all registered PHP functions.
unsafe extern "C" fn invoke(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    let execute_data = ExecuteData::from_mut_ptr(execute_data);
    let return_value = Val::from_mut_ptr(return_value);

    let num_args = execute_data.common_num_args();
    let arg_info = execute_data.common_arg_info();

    let last_arg_info = arg_info.offset((num_args + 1) as isize);
    let translator = CallableTranslator {
        arg_info: *last_arg_info,
    };
    let handler = translator.callable;
    let handler = handler.as_ref().expect("handler is null");

    // Check arguments count.
    let num_args = execute_data.num_args() as usize;
    let required_num_args = execute_data.common_required_num_args() as usize;
    if num_args < required_num_args {
        let func_name = execute_data.func().get_name();
        let result = func_name
            .map(|func_name| {
                Err::<(), _>(ArgumentCountError::new(
                    func_name,
                    required_num_args,
                    num_args,
                ))
            })
            .map_err(crate::Error::Utf8);
        SetVal::set_val(result, return_value);
        return;
    }

    let mut arguments = execute_data.get_parameters_array();

    // TODO catch_unwind for call, translate some panic to throwing Error.
    handler.call(execute_data, &mut arguments, return_value);
}

pub const fn create_zend_arg_info(
    name: *const c_char,
    _pass_by_ref: bool,
) -> zend_internal_arg_info {
    #[cfg(phper_php_version = "8.0")]
    {
        use std::ptr::null_mut;
        zend_internal_arg_info {
            name,
            type_: zend_type {
                ptr: null_mut(),
                type_mask: 0,
            },
            default_value: null_mut(),
        }
    }

    #[cfg(any(
        phper_php_version = "7.4",
        phper_php_version = "7.3",
        phper_php_version = "7.2"
    ))]
    {
        zend_internal_arg_info {
            name,
            type_: 0 as crate::sys::zend_type,
            pass_by_reference: _pass_by_ref as zend_uchar,
            is_variadic: 0,
        }
    }

    #[cfg(any(phper_php_version = "7.1", phper_php_version = "7.0"))]
    {
        zend_internal_arg_info {
            name,
            class_name: std::ptr::null(),
            type_hint: 0,
            allow_null: 0,
            pass_by_reference: _pass_by_ref as zend_uchar,
            is_variadic: 0,
        }
    }
}
