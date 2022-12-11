// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_class_entry].

use crate::{
    arrays::ZArr,
    errors::{ClassNotFoundError, InitializeObjectError},
    functions::{Function, FunctionEntry, Method, MethodEntity},
    objects::{ExtendObject, StateObj, ZObj, ZObject},
    strings::ZStr,
    sys::*,
    types::Scalar,
    utils::ensure_end_with_zero,
    values::ZVal,
};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use phper_alloc::ToRefOwned;
use std::{
    any::{Any, TypeId},
    borrow::ToOwned,
    convert::TryInto,
    ffi::{c_void, CString},
    fmt::Debug,
    marker::PhantomData,
    mem::{size_of, zeroed, ManuallyDrop},
    os::raw::c_int,
    ptr::null_mut,
    rc::Rc,
};

/// Wrapper of [crate::sys::zend_class_entry].
#[repr(transparent)]
pub struct ClassEntry {
    inner: zend_class_entry,
    _p: PhantomData<*mut ()>,
}

impl ClassEntry {
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_class_entry) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_class_entry) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_class_entry) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_class_entry) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    pub const fn as_ptr(&self) -> *const zend_class_entry {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_class_entry {
        &mut self.inner
    }

    pub fn from_globals<'a>(class_name: impl AsRef<str>) -> crate::Result<&'a Self> {
        let name = class_name.as_ref();
        let ptr: *mut Self = find_global_class_entry_ptr(name).cast();
        unsafe {
            ptr.as_ref().ok_or_else(|| {
                crate::Error::ClassNotFound(ClassNotFoundError::new(name.to_string()))
            })
        }
    }

    /// Create the object from class and call `__construct` with arguments.
    pub fn new_object(&self, arguments: impl AsMut<[ZVal]>) -> crate::Result<ZObject> {
        let mut object = self.init_object()?;
        object.call_construct(arguments)?;
        Ok(object)
    }

    /// Create the object from class, without calling `__construct`, be careful
    /// when `__construct` is necessary.
    pub fn init_object(&self) -> crate::Result<ZObject> {
        unsafe {
            let ptr = self.as_ptr() as *mut _;
            let mut val = ZVal::default();
            if !phper_object_init_ex(val.as_mut_ptr(), ptr) {
                Err(InitializeObjectError::new(self.get_name().to_str()?.to_owned()).into())
            } else {
                let ptr = phper_z_obj_p(val.as_mut_ptr());
                Ok(ZObj::from_mut_ptr(ptr).to_ref_owned())
            }
        }
    }

    pub fn get_name(&self) -> &ZStr {
        unsafe { ZStr::from_ptr(self.inner.name) }
    }

    pub fn has_method(&self, method_name: &str) -> bool {
        unsafe {
            let function_table = ZArr::from_ptr(&self.inner.function_table);
            function_table.exists(method_name)
        }
    }

    #[allow(clippy::useless_conversion)]
    pub fn instance_of(&self, parent: &ClassEntry) -> bool {
        unsafe { phper_instanceof_function(self.as_ptr(), parent.as_ptr()) != false.into() }
    }
}

impl Debug for ClassEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ClassEntry")
            .field(&self.get_name().to_c_str())
            .finish()
    }
}

#[allow(clippy::useless_conversion)]
fn find_global_class_entry_ptr(name: impl AsRef<str>) -> *mut zend_class_entry {
    let name = name.as_ref();
    let name = name.to_lowercase();
    unsafe {
        phper_zend_hash_str_find_ptr(
            compiler_globals.class_table,
            name.as_ptr().cast(),
            name.len().try_into().unwrap(),
        )
        .cast()
    }
}

pub type StateConstructor<T> = dyn Fn() -> T;

pub struct ClassEntity<T> {
    class_name: CString,
    state_constructor: Rc<StateConstructor<T>>,
    method_entities: Vec<MethodEntity>,
    property_entities: Vec<PropertyEntity>,
    parent: Option<String>,
    _p: PhantomData<*mut ()>,
}

impl ClassEntity<()> {
    pub fn new(class_name: impl Into<String>) -> Self {
        Self::new_with_state_constructor(class_name, || ())
    }
}

impl<T: Default + 'static> ClassEntity<T> {
    pub fn new_with_default_state_constructor(class_name: impl Into<String>) -> Self {
        Self::new_with_state_constructor(class_name, Default::default)
    }
}

impl<T: 'static> ClassEntity<T> {
    pub fn new_with_state_constructor(
        class_name: impl Into<String>, state_constructor: impl Fn() -> T + 'static,
    ) -> Self {
        Self {
            class_name: ensure_end_with_zero(class_name),
            state_constructor: Rc::new(state_constructor),
            method_entities: Vec::new(),
            property_entities: Vec::new(),
            parent: None,
            _p: PhantomData,
        }
    }

    pub fn add_method<F, R>(
        &mut self, name: impl Into<String>, vis: Visibility, handler: F,
    ) -> &mut MethodEntity
    where
        F: Fn(&mut StateObj<T>, &mut [ZVal]) -> R + Send + Sync + 'static,
        R: Into<ZVal> + 'static,
    {
        self.method_entities
            .push(MethodEntity::new(name, Rc::new(Method::new(handler)), vis));
        self.method_entities.last_mut().unwrap()
    }

    pub fn add_static_method<F, R>(
        &mut self, name: impl Into<String>, vis: Visibility, handler: F,
    ) -> &mut MethodEntity
    where
        F: Fn(&mut [ZVal]) -> R + Send + Sync + 'static,
        R: Into<ZVal> + 'static,
    {
        let mut entity = MethodEntity::new(name, Rc::new(Function::new(handler)), vis);
        entity.r#static(true);
        self.method_entities.push(entity);
        self.method_entities.last_mut().unwrap()
    }

    /// Declare property.
    ///
    /// The argument `value` should be `Copy` because 'zend_declare_property'
    /// receive only scalar zval , otherwise will report fatal error:
    /// "Internal zvals cannot be refcounted".
    pub fn add_property(
        &mut self, name: impl Into<String>, visibility: Visibility, value: impl Into<Scalar>,
    ) {
        self.property_entities
            .push(PropertyEntity::new(name, visibility, value));
    }

    pub fn extends(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.parent = Some(name);
    }

    #[allow(clippy::useless_conversion)]
    pub(crate) unsafe fn init(&mut self) -> *mut zend_class_entry {
        let parent: *mut zend_class_entry = self
            .parent
            .as_mut()
            .map(|s| ClassEntry::from_globals(s).unwrap())
            .map(|entry| entry.as_ptr() as *mut _)
            .unwrap_or(null_mut());

        let class_ce = phper_init_class_entry_ex(
            self.class_name.as_ptr().cast(),
            self.class_name.as_bytes().len().try_into().unwrap(),
            self.function_entries(),
            Some(class_init_handler),
            parent.cast(),
        );

        *phper_get_create_object(class_ce) = Some(create_object);

        get_registered_class_type_map().insert(class_ce as usize, TypeId::of::<T>());

        class_ce
    }

    pub(crate) unsafe fn declare_properties(&self, ce: *mut zend_class_entry) {
        for property in &self.property_entities {
            property.declare(ce);
        }
    }

    unsafe fn function_entries(&self) -> *const zend_function_entry {
        let last_entry = self.take_state_constructor_into_function_entry();
        let mut methods = self
            .method_entities
            .iter()
            .map(|method| FunctionEntry::from_method_entity(method))
            .collect::<Vec<_>>();

        methods.push(zeroed::<zend_function_entry>());

        // Store the state constructor pointer to zend_class_entry
        methods.push(last_entry);

        Box::into_raw(methods.into_boxed_slice()).cast()
    }

    unsafe fn take_state_constructor_into_function_entry(&self) -> zend_function_entry {
        let mut entry = zeroed::<zend_function_entry>();
        let ptr = &mut entry as *mut _ as *mut ManuallyDrop<Rc<StateConstructor<T>>>;
        let state_constructor = ManuallyDrop::new(self.state_constructor.clone());
        ptr.write(state_constructor);
        entry
    }
}

unsafe extern "C" fn class_init_handler(
    class_ce: *mut zend_class_entry, argument: *mut c_void,
) -> *mut zend_class_entry {
    let parent = argument as *mut zend_class_entry;
    if parent.is_null() {
        zend_register_internal_class(class_ce)
    } else {
        zend_register_internal_class_ex(class_ce, parent)
    }
}

pub struct PropertyEntity {
    name: String,
    visibility: Visibility,
    value: Scalar,
}

impl PropertyEntity {
    pub fn new(name: impl Into<String>, visibility: Visibility, value: impl Into<Scalar>) -> Self {
        Self {
            name: name.into(),
            visibility,
            value: value.into(),
        }
    }

    #[allow(clippy::useless_conversion)]
    pub(crate) fn declare(&self, ce: *mut zend_class_entry) {
        let name = self.name.as_ptr().cast();
        let name_length = self.name.len().try_into().unwrap();
        let access_type = self.visibility as u32 as i32;

        unsafe {
            match &self.value {
                Scalar::Null => {
                    zend_declare_property_null(ce, name, name_length, access_type);
                }
                Scalar::Bool(b) => {
                    zend_declare_property_bool(ce, name, name_length, *b as zend_long, access_type);
                }
                Scalar::I64(i) => {
                    zend_declare_property_long(ce, name, name_length, *i, access_type);
                }
                Scalar::F64(f) => {
                    zend_declare_property_double(ce, name, name_length, *f, access_type);
                }
                Scalar::String(s) => {
                    // If the `ce` is `ZEND_INTERNAL_CLASS`, then the `zend_string` is allocated
                    // as persistent.
                    zend_declare_property_stringl(
                        ce,
                        name,
                        name_length,
                        s.as_ptr().cast(),
                        s.len().try_into().unwrap(),
                        access_type,
                    );
                }
                Scalar::Bytes(b) => {
                    zend_declare_property_stringl(
                        ce,
                        name,
                        name_length,
                        b.as_ptr().cast(),
                        b.len().try_into().unwrap(),
                        access_type,
                    );
                }
            }
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Visibility {
    #[default]
    Public = ZEND_ACC_PUBLIC,
    Protected = ZEND_ACC_PROTECTED,
    Private = ZEND_ACC_PRIVATE,
}

fn get_registered_class_type_map() -> &'static DashMap<usize, TypeId> {
    static MAP: OnceCell<DashMap<usize, TypeId>> = OnceCell::new();
    MAP.get_or_init(DashMap::new)
}

fn get_object_handlers() -> &'static zend_object_handlers {
    static HANDLERS: OnceCell<zend_object_handlers> = OnceCell::new();
    HANDLERS.get_or_init(|| unsafe {
        let mut handlers = std_object_handlers;
        handlers.offset = ExtendObject::offset() as c_int;
        handlers.free_obj = Some(free_object);
        handlers
    })
}

#[allow(clippy::useless_conversion)]
unsafe extern "C" fn create_object(ce: *mut zend_class_entry) -> *mut zend_object {
    // Alloc more memory size to store state data.
    let extend_object: *mut ExtendObject =
        phper_zend_object_alloc(size_of::<ExtendObject>().try_into().unwrap(), ce).cast();

    // Common initialize process.
    let object = ExtendObject::as_mut_object(extend_object);
    zend_object_std_init(object, ce);
    object_properties_init(object, ce);
    rebuild_object_properties(object);
    object.handlers = get_object_handlers();

    // Get state constructor.
    let mut func_ptr = (*ce).info.internal.builtin_functions;
    while !(*func_ptr).fname.is_null() {
        func_ptr = func_ptr.offset(1);
    }
    func_ptr = func_ptr.offset(1);
    let state_constructor = func_ptr as *const ManuallyDrop<Box<StateConstructor<Box<dyn Any>>>>;
    let state_constructor = state_constructor.read();

    // Call the state constructor.
    let data: Box<dyn Any> = state_constructor();
    *ExtendObject::as_mut_state(extend_object) = ManuallyDrop::new(data);

    object
}

unsafe extern "C" fn free_object(object: *mut zend_object) {
    // Drop the state.
    let extend_object = ExtendObject::fetch_ptr(object);
    ExtendObject::drop_state(extend_object);

    // Original destroy call.
    zend_object_std_dtor(object);
}
