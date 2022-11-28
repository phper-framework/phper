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
    functions::{Argument, Function, FunctionEntity, FunctionEntry, Method},
    objects::{ExtendObject, StatefulObj, ZObj, ZObject},
    strings::ZStr,
    sys::*,
    types::Scalar,
    values::ZVal,
};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use phper_alloc::ToRefOwned;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    mem::{size_of, zeroed, ManuallyDrop},
    os::raw::c_int,
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};

pub trait Classifiable {
    fn state_constructor(&self) -> Box<StatefulConstructor<Box<dyn Any>>>;
    fn state_type_id(&self) -> TypeId;
    fn class_name(&self) -> &str;
    fn methods(&mut self) -> &mut [FunctionEntity];
    fn properties(&mut self) -> &mut [PropertyEntity];
    fn parent(&self) -> Option<&str>;
}

pub type StatefulConstructor<T> = dyn Fn() -> T + Send + Sync;

pub struct StatefulClass<T: Send + 'static> {
    class_name: String,
    state_constructor: Arc<StatefulConstructor<T>>,
    pub(crate) method_entities: Vec<FunctionEntity>,
    pub(crate) property_entities: Vec<PropertyEntity>,
    pub(crate) parent: Option<String>,
    _p: PhantomData<T>,
}

impl StatefulClass<()> {
    pub fn new(class_name: impl Into<String>) -> Self {
        Self::new_with_state_constructor(class_name, || ())
    }
}

impl<T: Default + Send + 'static> StatefulClass<T> {
    pub fn new_with_default_state(class_name: impl Into<String>) -> Self {
        Self::new_with_state_constructor(class_name, Default::default)
    }
}

impl<T: Send + 'static> StatefulClass<T> {
    pub fn new_with_state_constructor(
        class_name: impl Into<String>, state_constructor: impl Fn() -> T + Send + Sync + 'static,
    ) -> Self {
        Self {
            class_name: class_name.into(),
            state_constructor: Arc::new(state_constructor),
            method_entities: Vec::new(),
            property_entities: Vec::new(),
            parent: None,
            _p: Default::default(),
        }
        // let ptr = &dyn_class.data_constructor as *const _ as usize;
        // dyn_class.add_property(DATA_CONSTRUCTOR_PROPERTY_NAME,
        // ptr.to_string());
    }

    pub fn add_method<F, R>(
        &mut self, name: impl Into<String>, vis: Visibility, handler: F, arguments: Vec<Argument>,
    ) where
        F: Fn(&mut StatefulObj<T>, &mut [ZVal]) -> R + Send + Sync + 'static,
        R: Into<ZVal> + 'static,
    {
        self.method_entities.push(FunctionEntity::new(
            name,
            Box::new(Method::new(handler)),
            arguments,
            Some(vis),
            Some(false),
        ));
    }

    pub fn add_static_method<F, R>(
        &mut self, name: impl Into<String>, vis: Visibility, handler: F, arguments: Vec<Argument>,
    ) where
        F: Fn(&mut [ZVal]) -> R + Send + Sync + 'static,
        R: Into<ZVal> + 'static,
    {
        self.method_entities.push(FunctionEntity::new(
            name,
            Box::new(Function::new(handler)),
            arguments,
            Some(vis),
            Some(true),
        ));
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
}

impl<T: Send> Classifiable for StatefulClass<T> {
    fn state_constructor(&self) -> Box<StatefulConstructor<Box<dyn Any>>> {
        let sc = self.state_constructor.clone();
        Box::new(move || Box::new(sc()))
    }

    fn state_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn class_name(&self) -> &str {
        &self.class_name
    }

    fn methods(&mut self) -> &mut [FunctionEntity] {
        &mut self.method_entities
    }

    fn properties(&mut self) -> &mut [PropertyEntity] {
        &mut self.property_entities
    }

    fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }
}

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
}

fn find_global_class_entry_ptr(name: impl AsRef<str>) -> *mut zend_class_entry {
    let name = name.as_ref();
    let name = name.to_lowercase();
    unsafe {
        phper_zend_hash_str_find_ptr(
            compiler_globals.class_table,
            name.as_ptr().cast(),
            name.len(),
        )
        .cast()
    }
}

pub struct ClassEntity {
    pub(crate) name: String,
    pub(crate) entry: AtomicPtr<ClassEntry>,
    pub(crate) classifiable: Box<dyn Classifiable>,
    pub(crate) function_entries: OnceCell<AtomicPtr<FunctionEntry>>,
}

impl ClassEntity {
    pub(crate) unsafe fn new(classifiable: impl Classifiable + 'static) -> Self {
        Self {
            name: classifiable.class_name().to_string(),
            entry: AtomicPtr::new(null_mut()),
            classifiable: Box::new(classifiable),
            function_entries: Default::default(),
        }
    }

    pub(crate) unsafe fn init(&mut self) {
        let mut class_ce = phper_init_class_entry_ex(
            self.name.as_ptr().cast(),
            self.name.len(),
            self.function_entries().load(Ordering::SeqCst).cast(),
        );

        let parent: Option<&ClassEntry> = self
            .classifiable
            .parent()
            .map(|s| ClassEntry::from_globals(s).unwrap());

        let class: *mut ClassEntry = match parent {
            Some(parent) => {
                zend_register_internal_class_ex(&mut class_ce, parent.as_ptr() as *mut _).cast()
            }
            None => zend_register_internal_class(&mut class_ce).cast(),
        };
        self.entry.store(class.cast(), Ordering::SeqCst);

        *phper_get_create_object(class.cast()) = Some(create_object);

        get_registered_class_type_map().insert(class as usize, self.classifiable.state_type_id());
    }

    pub(crate) unsafe fn declare_properties(&mut self) {
        let properties = self.classifiable.properties();
        for property in properties {
            property.declare(self.entry.load(Ordering::SeqCst).cast());
        }
    }

    unsafe fn function_entries(&mut self) -> &AtomicPtr<FunctionEntry> {
        let last_entry = self.take_classifiable_into_function_entry();
        let methods = &*self.classifiable.methods();

        self.function_entries.get_or_init(|| {
            let mut methods = methods
                .iter()
                .map(|method| method.entry())
                .collect::<Vec<_>>();

            methods.push(zeroed::<zend_function_entry>());

            // Store the classifiable pointer to zend_class_entry
            methods.push(last_entry);

            let entry = Box::into_raw(methods.into_boxed_slice()).cast();
            AtomicPtr::new(entry)
        })
    }

    unsafe fn take_classifiable_into_function_entry(&self) -> zend_function_entry {
        let mut entry = zeroed::<zend_function_entry>();
        let ptr = &mut entry as *mut _ as *mut ManuallyDrop<Box<StatefulConstructor<Box<dyn Any>>>>;
        let state_constructor = ManuallyDrop::new(self.classifiable.state_constructor());
        ptr.write(state_constructor);
        entry
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

    pub(crate) fn declare(&self, ce: *mut zend_class_entry) {
        let name = self.name.as_ptr().cast();
        let name_length = self.name.len();
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
                        s.len(),
                        access_type,
                    );
                }
                Scalar::Bytes(b) => {
                    zend_declare_property_stringl(
                        ce,
                        name,
                        name_length,
                        b.as_ptr().cast(),
                        b.len(),
                        access_type,
                    );
                }
            }
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Visibility {
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

unsafe extern "C" fn create_object(ce: *mut zend_class_entry) -> *mut zend_object {
    // Alloc more memory size to store state data.
    let extend_object: *mut ExtendObject =
        phper_zend_object_alloc(size_of::<ExtendObject>(), ce).cast();

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
    let state_constructor = func_ptr as *const ManuallyDrop<Box<StatefulConstructor<Box<dyn Any>>>>;
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
