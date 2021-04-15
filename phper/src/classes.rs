use crate::{
    functions::{invoke, Argument, Callable, FunctionEntity, FunctionEntry, Method},
    sys::*,
    values::Val,
};
use once_cell::sync::OnceCell;
use std::{
    mem::zeroed,
    os::raw::c_int,
    ptr::{null, null_mut},
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc, Once, RwLock,
    },
};

pub trait Class: Send + Sync {
    fn methods(&self) -> &[FunctionEntity];
    fn properties(&self) -> &[PropertyEntity];
    fn parent(&self) -> Option<&str>;
}

pub struct StdClass {
    pub(crate) method_entities: Vec<FunctionEntity>,
    pub(crate) property_entities: Vec<PropertyEntity>,
    pub(crate) parent: Option<String>,
}

impl StdClass {
    pub fn new() -> Self {
        Self {
            method_entities: Vec::new(),
            property_entities: Vec::new(),
            parent: None,
        }
    }

    pub fn add_method(
        &mut self,
        name: impl ToString,
        handler: impl Method + 'static,
        arguments: Vec<Argument>,
    ) {
        self.method_entities.push(FunctionEntity::new(
            name,
            Callable::Method(Box::new(handler)),
            arguments,
        ));
    }

    pub fn add_property(&mut self, name: impl ToString, value: i32) {
        self.property_entities
            .push(PropertyEntity::new(name, value));
    }

    pub fn extends(&mut self, name: impl ToString) {
        let mut name = name.to_string();
        self.parent = Some(name);
    }
}

impl Class for StdClass {
    fn methods(&self) -> &[FunctionEntity] {
        &self.method_entities
    }

    fn properties(&self) -> &[PropertyEntity] {
        &self.property_entities
    }

    fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }
}

#[repr(transparent)]
pub struct ClassEntry {
    inner: zend_class_entry,
}

impl ClassEntry {
    pub fn as_mut(&mut self) -> *mut zend_class_entry {
        &mut self.inner
    }
}

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

            let parent = self.class.parent().map(|s| match s {
                "Exception" | "\\Exception" => zend_ce_exception,
                _ => todo!(),
            });

            let ptr = match parent {
                Some(parent) => zend_register_internal_class_ex(&mut class_ce, parent).cast(),
                None => zend_register_internal_class(&mut class_ce).cast(),
            };
            self.entry.store(ptr, Ordering::SeqCst);
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
                .map(|method| method.entry())
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
