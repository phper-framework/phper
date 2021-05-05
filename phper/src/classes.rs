use crate::{
    functions::{Argument, Callable, FunctionEntity, FunctionEntry, Method},
    sys::*,
    utils::ensure_end_with_zero,
    ClassNotFoundError,
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

    pub fn add_property(&mut self, name: impl ToString, value: String) {
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
    pub fn from_globals<'a>(class_name: impl AsRef<str>) -> Result<&'a Self, ClassNotFoundError> {
        let name = class_name.as_ref();
        let ptr: *mut Self = find_global_class_entry_ptr(name).cast();
        unsafe {
            ptr.as_ref()
                .ok_or_else(|| ClassNotFoundError::new(name.to_string()))
        }
    }

    pub fn from_ptr<'a>(ptr: *const zend_class_entry) -> &'a Self {
        unsafe { (ptr as *const Self).as_ref() }.expect("ptr should not be null")
    }

    pub fn as_ptr(&self) -> *const zend_class_entry {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_class_entry {
        &mut self.inner
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
            let value = ensure_end_with_zero(property.value.clone());
            zend_declare_property_string(
                self.entry.load(Ordering::SeqCst).cast(),
                property.name.as_ptr().cast(),
                property.name.len(),
                value.as_ptr().cast(),
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

pub struct PropertyEntity {
    name: String,
    // TODO to be a SetVal
    value: String,
}

impl PropertyEntity {
    pub fn new(name: impl ToString, value: String) -> Self {
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
