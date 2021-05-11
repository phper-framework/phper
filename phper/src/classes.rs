//! Apis relate to [crate::sys::zend_class_entry].

use crate::{
    errors::{ClassNotFoundError, Throwable},
    functions::{Argument, FunctionEntity, FunctionEntry, Method},
    objects::Object,
    sys::*,
    utils::ensure_end_with_zero,
    values::{SetVal, Val},
};
use once_cell::sync::OnceCell;
use std::{
    convert::Infallible,
    marker::PhantomData,
    mem::zeroed,
    os::raw::c_int,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

pub trait Classifiable {
    fn class_name(&self) -> &str;
    fn methods(&mut self) -> &mut [FunctionEntity];
    fn properties(&mut self) -> &mut [PropertyEntity];
    fn parent(&self) -> Option<&str>;
}

pub struct DynamicClass<T: Send + Sync + 'static> {
    class_name: String,
    data_constructor: Box<dyn FnOnce(&mut Object<T>) -> Result<T, Box<dyn Throwable>>>,
    pub(crate) method_entities: Vec<FunctionEntity>,
    pub(crate) property_entities: Vec<PropertyEntity>,
    pub(crate) parent: Option<String>,
    _p: PhantomData<T>,
}

impl DynamicClass<()> {
    pub fn new(class_name: impl ToString) -> Self {
        Self::new_with_constructor(class_name, |_| Ok::<_, Infallible>(()))
    }
}

impl<T: Default + Send + Sync + 'static> DynamicClass<T> {
    pub fn new_with_default(class_name: impl ToString) -> Self {
        Self::new_with_constructor(class_name, |_| Ok::<_, Infallible>(Default::default()))
    }
}

impl<T: Send + Sync + 'static> DynamicClass<T> {
    pub fn new_with_constructor<F, E>(class_name: impl ToString, data_constructor: F) -> Self
    where
        F: FnOnce(&mut Object<T>) -> Result<T, E> + 'static,
        E: Throwable + 'static,
    {
        Self {
            class_name: class_name.to_string(),
            data_constructor: Box::new(|o| data_constructor(o).map_err(|e| Box::new(e) as _)),
            method_entities: Vec::new(),
            property_entities: Vec::new(),
            parent: None,
            _p: Default::default(),
        }
    }

    pub fn add_method<F, R>(&mut self, name: impl ToString, handler: F, arguments: Vec<Argument>)
    where
        F: Fn(&mut Object<T>, &mut [Val]) -> R + Send + Sync + 'static,
        R: SetVal + 'static,
    {
        self.method_entities.push(FunctionEntity::new(
            name,
            Box::new(Method::new(handler)),
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

impl<T: Send + Sync> Classifiable for DynamicClass<T> {
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
    pub(crate) class: Box<dyn Classifiable>,
    pub(crate) function_entries: OnceCell<AtomicPtr<FunctionEntry>>,
}

impl ClassEntity {
    pub(crate) unsafe fn new(class: impl Classifiable + 'static) -> Self {
        Self {
            name: class.class_name().to_string(),
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

        let parent = self
            .class
            .parent()
            .map(|s| ClassEntry::from_globals(s).unwrap());

        let ptr = match parent {
            Some(parent) => {
                zend_register_internal_class_ex(&mut class_ce, parent.as_ptr() as *mut _).cast()
            }
            None => zend_register_internal_class(&mut class_ce).cast(),
        };
        self.entry.store(ptr, Ordering::SeqCst);

        // let methods = self.class.methods();
        // for method in methods {
        //     match &method.handler {
        //         Callable::Method(_, class) => {
        //             class.store(ptr, Ordering::SeqCst);
        //         }
        //         _ => unreachable!(),
        //     }
        // }
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
