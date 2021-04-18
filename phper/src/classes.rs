use crate::{
    functions::{Argument, Callable, FunctionEntity, FunctionEntry, Method},
    sys::*,
    values::{SetVal, Val},
};
use once_cell::sync::OnceCell;
use std::{
    mem::zeroed,
    os::raw::c_int,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

pub trait Class: Send + Sync {
    fn methods(&mut self) -> &mut [FunctionEntity];
    fn properties(&mut self) -> &mut [PropertyEntity];
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
            Callable::Method(Box::new(handler), AtomicPtr::new(null_mut())),
            arguments,
        ));
    }

    pub fn add_property(
        &mut self,
        name: impl ToString,
        value: impl SetVal + Send + Sync + 'static,
    ) {
        self.property_entities
            .push(PropertyEntity::new(name, value));
    }

    pub fn extends(&mut self, name: impl ToString) {
        let name = name.to_string();
        self.parent = Some(name);
    }
}

impl Class for StdClass {
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
    pub(crate) function_entries: OnceCell<AtomicPtr<FunctionEntry>>,
}

impl ClassEntity {
    pub(crate) unsafe fn new(name: impl ToString, class: impl Class + 'static) -> Self {
        Self {
            name: name.to_string(),
            entry: AtomicPtr::new(null_mut()),
            class: Box::new(class),
            function_entries: Default::default(),
        }
    }

    pub(crate) unsafe fn init(&mut self) {
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

        let methods = self.class.methods();
        for method in methods {
            match &method.handler {
                Callable::Method(_, class) => {
                    class.store(ptr, Ordering::SeqCst);
                }
                _ => unreachable!(),
            }
        }
    }

    pub(crate) unsafe fn declare_properties(&mut self) {
        let properties = self.class.properties();
        for property in properties {
            let mut val = Val::null();
            val.set(&property.value);
            zend_declare_property(
                self.entry.load(Ordering::SeqCst).cast(),
                property.name.as_ptr().cast(),
                property.name.len(),
                val.as_mut(),
                Visibility::Public as c_int,
            );
        }
    }

    unsafe fn function_entries(&mut self) -> &AtomicPtr<FunctionEntry> {
        let methods = &*self.class.methods();

        self.function_entries.get_or_init(|| {
            let mut methods = methods
                .iter()
                .map(|method| method.entry())
                .collect::<Vec<_>>();
            methods.push(zeroed::<zend_function_entry>());
            let entry = Box::into_raw(methods.into_boxed_slice()).cast();
            AtomicPtr::new(entry)
        })
    }
}

pub struct This {
    val: *mut Val,
    class: *mut ClassEntry,
}

impl This {
    pub(crate) fn new<'a>(val: *mut Val, class: *mut ClassEntry) -> This {
        assert!(!val.is_null());
        assert!(!class.is_null());
        Self { val, class }
    }

    pub fn get_property(&self, name: impl AsRef<str>) -> &mut Val {
        let name = name.as_ref();

        let prop = unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_read_property(
                    self.class as *mut _,
                    (*self.val).inner.value.obj,
                    name.as_ptr().cast(),
                    name.len(),
                    false.into(),
                    null_mut(),
                )
            }

            #[cfg(phper_major_version = "7")]
            {
                zend_read_property(
                    self.class as *mut _,
                    self.val as *mut _,
                    name.as_ptr().cast(),
                    name.len(),
                    false.into(),
                    null_mut(),
                )
            }
        };

        unsafe { Val::from_mut(prop) }
    }
}

pub struct PropertyEntity {
    name: String,
    value: Box<dyn SetVal + Send + Sync>,
}

impl PropertyEntity {
    pub fn new(name: impl ToString, value: impl SetVal + Send + Sync + 'static) -> Self {
        Self {
            name: name.to_string(),
            value: Box::new(value),
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
