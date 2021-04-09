use std::{
    mem::zeroed,
    os::raw::c_int,
    ptr::{null, null_mut},
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc, Once, RwLock,
    },
};

use once_cell::sync::OnceCell;

use crate::{
    functions::{method_invoke, FunctionEntry},
    sys::{
        phper_init_class_entry_ex, zend_class_entry, zend_declare_property_long,
        zend_function_entry, zend_internal_arg_info, zend_register_internal_class, zval,
        ZEND_ACC_PRIVATE, ZEND_ACC_PROTECTED, ZEND_ACC_PUBLIC,
    },
    values::Val,
};

pub trait Method: Send + Sync {
    fn call(&self, this: &mut This, arguments: &mut [Val], return_value: &mut Val);
}

impl<F> Method for F
where
    F: Fn(&mut This) + Send + Sync,
{
    fn call(&self, this: &mut This, _arguments: &mut [Val], _return_value: &mut Val) {
        self(this)
    }
}

pub struct MethodEntity {
    pub(crate) name: String,
    pub(crate) handler: Box<dyn Method>,
}

impl MethodEntity {
    pub fn new(name: impl ToString, handler: impl Method + 'static) -> Self {
        let mut name = name.to_string();
        name.push('\0');

        Self {
            name,
            handler: Box::new(handler),
        }
    }

    unsafe fn function_entry(&self) -> zend_function_entry {
        let mut infos = Vec::new();
        infos.push(zeroed::<zend_internal_arg_info>());

        let mut last_arg_info = zeroed::<zend_internal_arg_info>();
        last_arg_info.name = &self.handler as *const _ as *mut _;
        infos.push(last_arg_info);

        zend_function_entry {
            fname: self.name.as_ptr().cast(),
            handler: Some(method_invoke),
            arg_info: Box::into_raw(infos.into_boxed_slice()).cast(),
            num_args: 0,
            flags: 0,
        }
    }
}

pub trait Class: Send + Sync {
    fn methods(&self) -> &[MethodEntity];
    fn properties(&self) -> &[PropertyEntity];
}

pub struct StdClass {
    pub(crate) method_entities: Vec<MethodEntity>,
    pub(crate) property_entities: Vec<PropertyEntity>,
}

impl StdClass {
    pub fn new() -> Self {
        Self {
            method_entities: Vec::new(),
            property_entities: Vec::new(),
        }
    }

    pub fn add_method(&mut self, name: impl ToString, handler: impl Method + 'static) {
        self.method_entities.push(MethodEntity::new(name, handler));
    }

    pub fn add_property(&mut self, name: impl ToString, value: i32) {
        self.property_entities
            .push(PropertyEntity::new(name, value));
    }
}

impl Class for StdClass {
    fn methods(&self) -> &[MethodEntity] {
        &self.method_entities
    }

    fn properties(&self) -> &[PropertyEntity] {
        &self.property_entities
    }
}

#[repr(transparent)]
pub struct ClassEntry {
    inner: zend_class_entry,
}

impl ClassEntry {}

pub struct ClassEntity {
    pub(crate) name: String,
    pub(crate) entry: AtomicPtr<ClassEntry>,
    pub(crate) class: Box<dyn Class>,
    pub(crate) init_once: Once,
    pub(crate) function_entries: OnceCell<AtomicPtr<FunctionEntry>>,
}

impl ClassEntity {
    pub(crate) unsafe fn new(name: impl ToString, class: impl Class + 'static) -> Self {
        Self {
            name: name.to_string(),
            entry: AtomicPtr::new(null_mut()),
            class: Box::new(class),
            init_once: Once::new(),
            function_entries: Default::default(),
        }
    }

    pub(crate) unsafe fn init(&self) {
        self.init_once.call_once(|| {
            let mut class_ce = phper_init_class_entry_ex(
                self.name.as_ptr().cast(),
                self.name.len(),
                self.function_entries().load(Ordering::SeqCst).cast(),
            );
            self.entry.store(
                zend_register_internal_class(&mut class_ce).cast(),
                Ordering::SeqCst,
            );
        });
    }

    pub(crate) unsafe fn declare_properties(&self) {
        let properties = self.class.properties();
        for property in properties {
            zend_declare_property_long(
                self.entry.load(Ordering::SeqCst).cast(),
                property.name.as_ptr().cast(),
                property.name.len(),
                property.value.into(),
                Visibility::Public as c_int,
            );
        }
    }

    unsafe fn function_entries(&self) -> &AtomicPtr<FunctionEntry> {
        self.function_entries.get_or_init(|| {
            let mut methods = self
                .class
                .methods()
                .iter()
                .map(|method| method.function_entry())
                .collect::<Vec<_>>();
            methods.push(zeroed::<zend_function_entry>());
            let entry = Box::into_raw(methods.into_boxed_slice()).cast();
            AtomicPtr::new(entry)
        })
    }
}

#[repr(transparent)]
pub struct This {
    inner: zval,
}

impl This {
    #[inline]
    pub unsafe fn from_mut<'a>(ptr: *mut zval) -> &'a mut Self {
        assert!(!ptr.is_null(), "ptr should not be null");
        &mut *(ptr as *mut Self)
    }
}

pub struct PropertyEntity {
    name: String,
    value: i32,
}

impl PropertyEntity {
    pub fn new(name: impl ToString, value: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Visibility {
    Public = ZEND_ACC_PUBLIC,
    Protected = ZEND_ACC_PROTECTED,
    Private = ZEND_ACC_PRIVATE,
}
