//! Apis relate to [crate::sys::zend_class_entry].

use std::{
    any::Any,
    collections::HashMap,
    convert::Infallible,
    marker::PhantomData,
    mem::{size_of, zeroed, ManuallyDrop},
    os::raw::c_int,
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};

use dashmap::DashMap;
use once_cell::sync::OnceCell;

use phper_alloc::EBox;

use crate::{
    errors::{ClassNotFoundError, Throwable},
    functions::{Argument, FunctionEntity, FunctionEntry, Method},
    objects::{ExtendObject, Object},
    sys::*,
    utils::ensure_end_with_zero,
    values::{SetVal, Val},
};
use std::rc::Rc;

pub trait Classifiable {
    fn state_constructor(&self) -> Box<StateConstructor<Box<dyn Any>>>;
    fn class_name(&self) -> &str;
    fn methods(&mut self) -> &mut [FunctionEntity];
    fn properties(&mut self) -> &mut [PropertyEntity];
    fn parent(&self) -> Option<&str>;
}

pub type StateConstructor<T> = dyn Fn() -> Result<T, Box<dyn Throwable>> + Send + Sync;

pub struct DynamicClass<T: Send + Sync + 'static> {
    class_name: String,
    state_constructor: Arc<StateConstructor<T>>,
    pub(crate) method_entities: Vec<FunctionEntity>,
    pub(crate) property_entities: Vec<PropertyEntity>,
    pub(crate) parent: Option<String>,
    _p: PhantomData<T>,
}

impl DynamicClass<()> {
    pub fn new(class_name: impl ToString) -> Self {
        Self::new_with_constructor(class_name, || Ok::<_, Infallible>(()))
    }
}

impl<T: Default + Send + Sync + 'static> DynamicClass<T> {
    pub fn new_with_default(class_name: impl ToString) -> Self {
        Self::new_with_constructor(class_name, || Ok::<_, Infallible>(Default::default()))
    }
}

impl<T: Send + Sync + 'static> DynamicClass<Option<T>> {
    pub fn new_with_none(class_name: impl ToString) -> Self {
        Self::new_with_constructor(class_name, || Ok::<_, Infallible>(None))
    }
}

impl<T: Send + Sync + 'static> DynamicClass<T> {
    pub fn new_with_constructor<F, E>(class_name: impl ToString, state_constructor: F) -> Self
    where
        F: Fn() -> Result<T, E> + Send + Sync + 'static,
        E: Throwable + 'static,
    {
        let mut dyn_class = Self {
            class_name: class_name.to_string(),
            state_constructor: Arc::new(move || state_constructor().map_err(|e| Box::new(e) as _)),
            method_entities: Vec::new(),
            property_entities: Vec::new(),
            parent: None,
            _p: Default::default(),
        };

        // let ptr = &dyn_class.data_constructor as *const _ as usize;
        // dyn_class.add_property(DATA_CONSTRUCTOR_PROPERTY_NAME, ptr.to_string());

        dyn_class
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
    fn state_constructor(&self) -> Box<StateConstructor<Box<dyn Any>>> {
        let sc = self.state_constructor.clone();
        Box::new(move || sc().map(|x| Box::new(x) as _))
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
///
/// TODO Add generic type.
#[repr(transparent)]
pub struct ClassEntry {
    inner: zend_class_entry,
}

impl ClassEntry {
    // TODO After added generic type, Check generic type is relate to class_name, otherwise return error.
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

    pub(crate) fn create_object<T>(&self) -> EBox<Object<T>> {
        unsafe {
            let f = self.inner.__bindgen_anon_2.create_object.unwrap();
            let object = f(self.as_ptr() as *mut _);
            EBox::from_raw(object.cast())
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

        let parent = self
            .classifiable
            .parent()
            .map(|s| ClassEntry::from_globals(s).unwrap());

        let class: *mut ClassEntry = match parent {
            Some(parent) => {
                zend_register_internal_class_ex(&mut class_ce, parent.as_ptr() as *mut _).cast()
            }
            None => zend_register_internal_class(&mut class_ce).cast(),
        };
        self.entry.store(class, Ordering::SeqCst);

        (*class).inner.__bindgen_anon_2.create_object = Some(create_object);

        // let classifiable = self.classifiable.clone();
        // get_class_constructor_map().insert(class as usize, Box::new(move || classifiable.state_constructor()));

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
        let properties = self.classifiable.properties();
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
        let ptr = &mut entry as *mut _ as *mut ManuallyDrop<Box<StateConstructor<Box<Any>>>>;
        let state_constructor = ManuallyDrop::new(self.classifiable.state_constructor());
        ptr.write(state_constructor);
        entry
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

fn get_object_handlers() -> &'static zend_object_handlers {
    static OBJECT_HANDLERS: OnceCell<zend_object_handlers> = OnceCell::new();
    OBJECT_HANDLERS.get_or_init(|| unsafe {
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
    let state_constructor = func_ptr as *const ManuallyDrop<Box<StateConstructor<Box<Any>>>>;
    let state_constructor = state_constructor.read();

    // Call the state constructor.
    // TODO Throw an exception rather than unwrap.
    let data: Box<dyn Any> = state_constructor().unwrap();
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
