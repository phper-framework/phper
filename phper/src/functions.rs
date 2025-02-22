// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_function_entry].
//!
//! TODO Add lambda.

use crate::{
    classes::{ClassEntry, RawVisibility, Visibility},
    errors::{ArgumentCountError, ExceptionGuard, ThrowObject, Throwable, throw},
    modules::global_module,
    objects::{StateObj, ZObj, ZObject},
    strings::{ZStr, ZString},
    sys::*,
    types::TypeInfo,
    utils::ensure_end_with_zero,
    values::{ExecuteData, ZVal},
};
use phper_alloc::ToRefOwned;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::{ManuallyDrop, size_of, transmute, zeroed},
    ptr::{self, null_mut},
    rc::Rc,
    slice,
};

/// Used to mark the arguments obtained by the invoke function as mysterious
/// codes from phper
const INVOKE_MYSTERIOUS_CODE: &[u8] = b"PHPER";

/// Used to find the handler in the invoke function.
pub(crate) type HandlerMap = HashMap<(Option<CString>, CString), Rc<dyn Callable>>;

pub(crate) trait Callable {
    fn call(&self, execute_data: &mut ExecuteData, arguments: &mut [ZVal], return_value: &mut ZVal);
}

pub(crate) struct Function<F, Z, E>(F, PhantomData<(Z, E)>);

impl<F, Z, E> Function<F, Z, E> {
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Z, E> Callable for Function<F, Z, E>
where
    F: Fn(&mut [ZVal]) -> Result<Z, E>,
    Z: Into<ZVal>,
    E: Throwable,
{
    fn call(&self, _: &mut ExecuteData, arguments: &mut [ZVal], return_value: &mut ZVal) {
        match (self.0)(arguments) {
            Ok(z) => {
                *return_value = z.into();
            }
            Err(e) => {
                unsafe {
                    throw(e);
                }
                *return_value = ().into();
            }
        }
    }
}

pub(crate) struct Method<F, Z, E, T>(F, PhantomData<(Z, E, T)>);

impl<F, Z, E, T> Method<F, Z, E, T> {
    pub(crate) fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Z, E, T: 'static> Callable for Method<F, Z, E, T>
where
    F: Fn(&mut StateObj<T>, &mut [ZVal]) -> Result<Z, E>,
    Z: Into<ZVal>,
    E: Throwable,
{
    fn call(
        &self, execute_data: &mut ExecuteData, arguments: &mut [ZVal], return_value: &mut ZVal,
    ) {
        let this = unsafe { execute_data.get_this_mut().unwrap().as_mut_state_obj() };
        match (self.0)(this, arguments) {
            Ok(z) => {
                *return_value = z.into();
            }
            Err(e) => {
                unsafe {
                    throw(e);
                }
                *return_value = ().into();
            }
        }
    }
}

/// Wrapper of [`zend_function_entry`].
#[repr(transparent)]
pub struct FunctionEntry {
    #[allow(dead_code)]
    inner: zend_function_entry,
}

impl FunctionEntry {
    pub(crate) unsafe fn from_function_entity(entity: &FunctionEntity) -> zend_function_entry {
        unsafe {
            Self::entry(
                &entity.name,
                &entity.arguments,
                entity.return_type.as_ref(),
                Some(entity.handler.clone()),
                None,
            )
        }
    }

    pub(crate) unsafe fn from_method_entity(entity: &MethodEntity) -> zend_function_entry {
        unsafe {
            Self::entry(
                &entity.name,
                &entity.arguments,
                entity.return_type.as_ref(),
                entity.handler.clone(),
                Some(entity.visibility),
            )
        }
    }

    /// Will leak memory
    unsafe fn entry(
        name: &CStr, arguments: &[Argument], return_type: Option<&ReturnType>,
        handler: Option<Rc<dyn Callable>>, visibility: Option<RawVisibility>,
    ) -> zend_function_entry {
        unsafe {
            let mut infos = Vec::new();

            let require_arg_count = arguments.iter().filter(|arg| arg.required).count();

            if let Some(return_type) = return_type {
                infos.push(phper_zend_begin_arg_with_return_type_info_ex(
                    return_type.ret_by_ref,
                    require_arg_count,
                    return_type.type_info.into_raw(),
                    return_type.allow_null,
                ));
            } else {
                infos.push(phper_zend_begin_arg_info_ex(false, require_arg_count));
            }

            for arg in arguments {
                infos.push(phper_zend_arg_info(
                    arg.pass_by_ref,
                    arg.name.as_ptr().cast(),
                ));
            }

            infos.push(zeroed::<zend_internal_arg_info>());

            // Will be checked in `invoke` function.
            infos.push(Self::create_mysterious_code());

            let raw_handler = handler.as_ref().map(|_| invoke as _);

            if let Some(handler) = handler {
                let translator = CallableTranslator {
                    callable: Rc::into_raw(handler),
                };
                let last_arg_info: zend_internal_arg_info = translator.internal_arg_info;
                infos.push(last_arg_info);
            }

            let flags = visibility.unwrap_or(Visibility::default() as u32);

            #[allow(clippy::needless_update)]
            zend_function_entry {
                fname: name.as_ptr().cast(),
                handler: raw_handler,
                arg_info: Box::into_raw(infos.into_boxed_slice()).cast(),
                num_args: arguments.len() as u32,
                flags,
                ..Default::default()
            }
        }
    }

    unsafe fn create_mysterious_code() -> zend_internal_arg_info {
        unsafe {
            let mut mysterious_code = [0u8; size_of::<zend_internal_arg_info>()];
            for (i, n) in INVOKE_MYSTERIOUS_CODE.iter().enumerate() {
                mysterious_code[i] = *n;
            }
            transmute(mysterious_code)
        }
    }
}

/// Builder for registering php function.
pub struct FunctionEntity {
    name: CString,
    handler: Rc<dyn Callable>,
    arguments: Vec<Argument>,
    return_type: Option<ReturnType>,
}

impl FunctionEntity {
    #[inline]
    pub(crate) fn new(name: impl Into<String>, handler: Rc<dyn Callable>) -> Self {
        FunctionEntity {
            name: ensure_end_with_zero(name),
            handler,
            arguments: Default::default(),
            return_type: None,
        }
    }

    /// Add single function argument info.
    #[inline]
    pub fn argument(&mut self, argument: Argument) -> &mut Self {
        self.arguments.push(argument);
        self
    }

    /// Add many function argument infos.
    #[inline]
    pub fn arguments(&mut self, arguments: impl IntoIterator<Item = Argument>) -> &mut Self {
        self.arguments.extend(arguments);
        self
    }

    /// Add return type info.
    #[inline]
    pub fn return_type(&mut self, return_type: ReturnType) -> &mut Self {
        self.return_type = Some(return_type);
        self
    }
}

/// Builder for registering class method.
pub struct MethodEntity {
    pub(crate) name: CString,
    pub(crate) handler: Option<Rc<dyn Callable>>,
    arguments: Vec<Argument>,
    visibility: RawVisibility,
    return_type: Option<ReturnType>,
}

impl MethodEntity {
    #[inline]
    pub(crate) fn new(
        name: impl Into<String>, handler: Option<Rc<dyn Callable>>, visibility: Visibility,
    ) -> Self {
        Self {
            name: ensure_end_with_zero(name),
            handler,
            visibility: visibility as RawVisibility,
            arguments: Default::default(),
            return_type: None,
        }
    }

    #[inline]
    pub(crate) fn set_vis_static(&mut self) -> &mut Self {
        self.visibility |= ZEND_ACC_STATIC;
        self
    }

    #[inline]
    pub(crate) fn set_vis_abstract(&mut self) -> &mut Self {
        self.visibility |= ZEND_ACC_ABSTRACT;
        self
    }

    /// Add single method argument info.
    #[inline]
    pub fn argument(&mut self, argument: Argument) -> &mut Self {
        self.arguments.push(argument);
        self
    }

    /// Add many method argument infos.
    #[inline]
    pub fn arguments(&mut self, arguments: impl IntoIterator<Item = Argument>) -> &mut Self {
        self.arguments.extend(arguments);
        self
    }

    /// Add return type info.
    #[inline]
    pub fn return_type(&mut self, return_type: ReturnType) -> &mut Self {
        self.return_type = Some(return_type);
        self
    }
}

/// Function or method argument info.
pub struct Argument {
    name: CString,
    pass_by_ref: bool,
    required: bool,
}

impl Argument {
    /// Indicate the argument is pass by value.
    pub fn by_val(name: impl Into<String>) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: false,
            required: true,
        }
    }

    /// Indicate the argument is pass by reference.
    pub fn by_ref(name: impl Into<String>) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: true,
            required: true,
        }
    }

    /// Indicate the argument is pass by value and is optional.
    pub fn by_val_optional(name: impl Into<String>) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: false,
            required: false,
        }
    }

    /// Indicate the argument is pass by reference nad is optional.
    pub fn by_ref_optional(name: impl Into<String>) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: true,
            required: false,
        }
    }
}

/// Function or method return type.
pub struct ReturnType {
    type_info: TypeInfo,
    ret_by_ref: bool,
    allow_null: bool,
}

impl ReturnType {
    /// Indicate the return type is return by value.
    #[inline]
    pub fn by_val(type_info: TypeInfo) -> Self {
        Self {
            type_info,
            ret_by_ref: false,
            allow_null: false,
        }
    }

    /// Indicate the return type is return by reference.
    #[inline]
    pub fn by_ref(type_info: TypeInfo) -> Self {
        Self {
            type_info,
            ret_by_ref: true,
            allow_null: false,
        }
    }

    /// Indicate the return type can be null.
    #[inline]
    pub fn allow_null(mut self) -> Self {
        self.allow_null = true;
        self
    }
}

/// Wrapper of [`zend_function`].
#[repr(transparent)]
pub struct ZFunc {
    inner: zend_function,
}

impl ZFunc {
    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    pub(crate) unsafe fn from_mut_ptr<'a>(ptr: *mut zend_function) -> &'a mut ZFunc {
        unsafe {
            let ptr = ptr as *mut Self;
            ptr.as_mut().expect("ptr shouldn't be null")
        }
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_function {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_function {
        &mut self.inner
    }

    /// Get the function name if exists.
    pub fn get_function_name(&self) -> Option<&ZStr> {
        unsafe {
            let s = phper_get_function_name(self.as_ptr());
            ZStr::try_from_ptr(s)
        }
    }

    /// Get the function or method fully-qualified name.
    pub fn get_function_or_method_name(&self) -> ZString {
        unsafe {
            let s = phper_get_function_or_method_name(self.as_ptr());
            ZString::from_raw(s)
        }
    }

    /// Get the function related class if exists.
    pub fn get_class(&self) -> Option<&ClassEntry> {
        unsafe {
            let ptr = self.inner.common.scope;
            if ptr.is_null() {
                None
            } else {
                Some(ClassEntry::from_ptr(self.inner.common.scope))
            }
        }
    }

    #[allow(clippy::useless_conversion)]
    pub(crate) fn call(
        &mut self, mut object: Option<&mut ZObj>, mut arguments: impl AsMut<[ZVal]>,
    ) -> crate::Result<ZVal> {
        let arguments = arguments.as_mut();
        let function_handler = self.as_mut_ptr();

        let object_ptr = object
            .as_mut()
            .map(|o| o.as_mut_ptr())
            .unwrap_or(null_mut());

        call_raw_common(|ret| unsafe {
            #[cfg(phper_major_version = "8")]
            {
                let class_ptr = object
                    .as_mut()
                    .map(|o| o.get_mut_class().as_mut_ptr())
                    .unwrap_or(null_mut());

                zend_call_known_function(
                    function_handler,
                    object_ptr,
                    class_ptr,
                    ret.as_mut_ptr(),
                    arguments.len() as u32,
                    arguments.as_mut_ptr().cast(),
                    null_mut(),
                );
            }
            #[cfg(phper_major_version = "7")]
            {
                use std::mem::size_of;

                let called_scope = {
                    let mut called_scope = object
                        .as_mut()
                        .map(|o| o.get_class().as_ptr() as *mut zend_class_entry)
                        .unwrap_or(null_mut());
                    if called_scope.is_null() {
                        called_scope = self.inner.common.scope;
                    }
                    called_scope
                };

                let mut fci = zend_fcall_info {
                    size: size_of::<zend_fcall_info>().try_into().unwrap(),
                    function_name: ZVal::from(()).into_inner(),
                    retval: ret.as_mut_ptr(),
                    params: arguments.as_mut_ptr().cast(),
                    object: object_ptr,
                    param_count: arguments.len() as u32,
                    no_separation: 1,
                    #[cfg(all(phper_major_version = "7", phper_minor_version = "0"))]
                    function_table: null_mut(),
                    #[cfg(all(phper_major_version = "7", phper_minor_version = "0"))]
                    symbol_table: null_mut(),
                };

                let mut fcc = zend_fcall_info_cache {
                    function_handler,
                    calling_scope: null_mut(),
                    called_scope,
                    object: object_ptr,
                    #[cfg(all(
                        phper_major_version = "7",
                        any(
                            phper_minor_version = "2",
                            phper_minor_version = "1",
                            phper_minor_version = "0",
                        )
                    ))]
                    initialized: 1,
                };

                zend_call_function(&mut fci, &mut fcc);
            }
        })
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
    unsafe {
        let execute_data = ExecuteData::from_mut_ptr(execute_data);
        let return_value = ZVal::from_mut_ptr(return_value);

        let num_args = execute_data.common_num_args();
        let arg_info = execute_data.common_arg_info();

        // should be mysterious code
        let mysterious_arg_info = arg_info.offset((num_args + 1) as isize);
        let mysterious_code = slice::from_raw_parts(
            mysterious_arg_info as *const u8,
            INVOKE_MYSTERIOUS_CODE.len(),
        );

        let handler = if mysterious_code == INVOKE_MYSTERIOUS_CODE {
            // hiddden real handler
            let last_arg_info = arg_info.offset((num_args + 2) as isize);
            let translator = CallableTranslator {
                arg_info: *last_arg_info,
            };
            let handler = translator.callable;
            handler.as_ref().expect("handler is null")
        } else {
            let function_name = execute_data
                .func()
                .get_function_name()
                .and_then(|name| name.to_c_str().ok())
                .map(CString::from);

            function_name
                .and_then(|function_name| {
                    let class_name = execute_data
                        .func()
                        .get_class()
                        .and_then(|cls| cls.get_name().to_c_str().ok())
                        .map(CString::from);

                    global_module()
                        .handler_map
                        .get(&(class_name, function_name))
                })
                .expect("invoke handler is not correct")
                .as_ref()
        };

        // Check arguments count.
        let num_args = execute_data.num_args();
        let required_num_args = execute_data.common_required_num_args();
        if num_args < required_num_args {
            let func_name = execute_data.func().get_function_or_method_name();
            let err: crate::Error = match func_name.to_str() {
                Ok(func_name) => {
                    ArgumentCountError::new(func_name.to_owned(), required_num_args, num_args)
                        .into()
                }
                Err(e) => e.into(),
            };
            throw(err);
            *return_value = ().into();
            return;
        }

        let mut arguments = execute_data.get_parameters_array();
        let arguments = arguments.as_mut_slice();

        handler.call(
            execute_data,
            transmute::<&mut [ManuallyDrop<ZVal>], &mut [ZVal]>(arguments),
            return_value,
        );
    }
}

/// Call user function by name.
///
/// # Examples
///
/// ```no_run
/// use phper::{arrays::ZArray, functions::call, values::ZVal};
///
/// fn json_encode() -> phper::Result<()> {
///     let mut arr = ZArray::new();
///     arr.insert("a", ZVal::from(1));
///     arr.insert("b", ZVal::from(2));
///     let ret = call("json_encode", &mut [ZVal::from(arr)])?;
///     assert_eq!(ret.expect_z_str()?.to_str(), Ok(r#"{"a":1,"b":2}"#));
///     Ok(())
/// }
/// ```
pub fn call(callable: impl Into<ZVal>, arguments: impl AsMut<[ZVal]>) -> crate::Result<ZVal> {
    let mut func = callable.into();
    call_internal(&mut func, None, arguments)
}

pub(crate) fn call_internal(
    func: &mut ZVal, mut object: Option<&mut ZObj>, mut arguments: impl AsMut<[ZVal]>,
) -> crate::Result<ZVal> {
    let func_ptr = func.as_mut_ptr();
    let arguments = arguments.as_mut();

    let mut object_val = object.as_mut().map(|obj| ZVal::from(obj.to_ref_owned()));

    call_raw_common(|ret| unsafe {
        phper_call_user_function(
            cg!(function_table),
            object_val
                .as_mut()
                .map(|o| o.as_mut_ptr())
                .unwrap_or(null_mut()),
            func_ptr,
            ret.as_mut_ptr(),
            arguments.len() as u32,
            arguments.as_mut_ptr().cast(),
        );
    })
}

/// call function with raw pointer.
/// call_fn parameters: (return_value)
pub(crate) fn call_raw_common(call_fn: impl FnOnce(&mut ZVal)) -> crate::Result<ZVal> {
    let _guard = ExceptionGuard::default();

    let mut ret = ZVal::default();

    call_fn(&mut ret);
    if ret.get_type_info().is_undef() {
        ret = ().into();
    }

    unsafe {
        if !eg!(exception).is_null() {
            #[allow(static_mut_refs)]
            let e = ptr::replace(&mut eg!(exception), null_mut());
            let obj = ZObject::from_raw(e);
            match ThrowObject::new(obj) {
                Ok(e) => return Err(e.into()),
                Err(e) => return Err(e.into()),
            }
        }
    }

    Ok(ret)
}
