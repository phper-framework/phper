//! Apis relate to [crate::sys::zend_function_entry].
//!
//! TODO Add lambda.

use std::{mem::zeroed, os::raw::c_char};

use crate::{
    alloc::EBox,
    classes::Visibility,
    errors::{ArgumentCountError, CallFunctionError, CallMethodError},
    objects::Object,
    strings::ZendString,
    sys::*,
    utils::ensure_end_with_zero,
    values::{ExecuteData, SetVal, Val},
};
use std::{marker::PhantomData, mem::size_of, ptr::null_mut};

pub(crate) trait Callable {
    fn call(&self, execute_data: &mut ExecuteData, arguments: &mut [Val], return_value: &mut Val);
}

pub(crate) struct Function<F, R>(F)
where
    F: Fn(&mut [Val]) -> R + Send + Sync,
    R: SetVal;

impl<F, R> Function<F, R>
where
    F: Fn(&mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F, R> Callable for Function<F, R>
where
    F: Fn(&mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    fn call(&self, _: &mut ExecuteData, arguments: &mut [Val], return_value: &mut Val) {
        let r = (self.0)(arguments);
        unsafe {
            r.set_val(return_value);
        }
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

impl<F, R, T: 'static> Callable for Method<F, R, T>
where
    F: Fn(&mut Object<T>, &mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    fn call(&self, execute_data: &mut ExecuteData, arguments: &mut [Val], return_value: &mut Val) {
        unsafe {
            let this = execute_data.get_this::<T>().unwrap();
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
    pub(crate) visibility: Option<Visibility>,
    pub(crate) r#static: Option<bool>,
}

impl FunctionEntity {
    pub(crate) fn new(
        name: impl ToString,
        handler: Box<dyn Callable>,
        arguments: Vec<Argument>,
        visibility: Option<Visibility>,
        r#static: Option<bool>,
    ) -> Self {
        let name = ensure_end_with_zero(name);
        FunctionEntity {
            name,
            handler,
            arguments,
            visibility,
            r#static,
        }
    }

    /// Will leak memory
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
        let last_arg_info: zend_internal_arg_info = translator.internal_arg_info;
        infos.push(last_arg_info);

        let flags = self.visibility.map(|v| v as u32).unwrap_or_default()
            | self
                .r#static
                .and_then(|v| if v { Some(ZEND_ACC_STATIC) } else { None })
                .unwrap_or_default();

        zend_function_entry {
            fname: self.name.as_ptr().cast(),
            handler: Some(invoke),
            arg_info: Box::into_raw(infos.into_boxed_slice()).cast(),
            num_args: self.arguments.len() as u32,
            flags,
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

    pub fn get_name(&self) -> EBox<ZendString> {
        unsafe {
            let s = phper_get_function_or_method_name(self.as_ptr());
            ZendString::from_raw(s)
        }
    }

    pub fn call_method<T: 'static>(
        &mut self,
        object: &mut Object<T>,
        arguments: &mut [Val],
    ) -> crate::Result<EBox<Val>> {
        let mut ret_val = EBox::new(Val::undef());

        let mut fci = zend_fcall_info {
            size: size_of::<zend_fcall_info>(),
            function_name: Val::undef().into_inner(),
            retval: ret_val.as_mut_ptr(),
            params: arguments.as_mut_ptr().cast(),
            object: object.as_mut_ptr(),
            param_count: arguments.len() as u32,
            #[cfg(phper_major_version = "8")]
            named_params: null_mut(),
            #[cfg(phper_major_version = "7")]
            no_separation: 1,
            #[cfg(all(phper_major_version = "7", phper_minor_version = "0"))]
            function_table: null_mut(),
            #[cfg(all(phper_major_version = "7", phper_minor_version = "0"))]
            symbol_table: null_mut(),
        };

        let called_scope = unsafe {
            let mut called_scope = object.get_class().as_ptr() as *mut zend_class_entry;
            if called_scope.is_null() {
                called_scope = self.inner.common.scope;
            }
            called_scope
        };

        let mut fcc = zend_fcall_info_cache {
            function_handler: self.as_mut_ptr(),
            calling_scope: null_mut(),
            called_scope,
            object: object.as_mut_ptr(),
            #[cfg(all(
                phper_major_version = "7",
                any(
                    phper_minor_version = "0",
                    phper_minor_version = "1",
                    phper_minor_version = "2",
                )
            ))]
            initialized: 1,
        };

        unsafe {
            if zend_call_function(&mut fci, &mut fcc) != ZEND_RESULT_CODE_SUCCESS
                || ret_val.get_type().is_undef()
            {
                Err(CallMethodError::new(
                    object.get_class().get_name().to_string()?,
                    self.get_name().to_string()?,
                )
                .into())
            } else {
                Ok(ret_val)
            }
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
    // TODO Use `zend_argument_count_error` rather than just throw an exception.
    let num_args = execute_data.num_args() as usize;
    let required_num_args = execute_data.common_required_num_args() as usize;
    if num_args < required_num_args {
        let func_name = execute_data.func().get_name();
        let result = func_name
            .to_string()
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

pub(crate) const fn create_zend_arg_info(
    name: *const c_char,
    _pass_by_ref: bool,
) -> zend_internal_arg_info {
    #[cfg(phper_php_version = "8.0")]
    {
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

/// Call user function by name.
///
/// # Examples
///
/// ```
/// use phper::{arrays::Array, functions::call, values::Val};
///
/// fn example() -> phper::Result<()> {
///     let mut arr = Array::new();
///     arr.insert("a", Val::new(1));
///     arr.insert("b", Val::new(2));
///     let ret = call("json_encode", &mut [Val::new(arr)])?;
///     assert_eq!(ret.as_string()?, r#"{"a":1,"b":2}"#);
///     Ok(())
/// }
/// ```
pub fn call(fn_name: &str, arguments: &mut [Val]) -> Result<EBox<Val>, CallFunctionError> {
    let mut func = Val::new(fn_name);
    let mut ret = EBox::new(Val::null());
    unsafe {
        if phper_call_user_function(
            compiler_globals.function_table,
            null_mut(),
            func.as_mut_ptr(),
            ret.as_mut_ptr(),
            arguments.len() as u32,
            arguments.as_mut_ptr().cast(),
        ) && !ret.get_type().is_undef()
        {
            Ok(ret)
        } else {
            Err(CallFunctionError::new(fn_name.to_owned()))
        }
    }
}
