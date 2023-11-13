// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_class_entry].

use crate::{
    arrays::ZArr,
    errors::{ClassNotFoundError, InitializeObjectError, Throwable},
    functions::{Function, FunctionEntry, Method, MethodEntity},
    objects::{StateObj, StateObject, ZObject},
    strings::ZStr,
    sys::*,
    types::Scalar,
    utils::ensure_end_with_zero,
    values::ZVal,
};
use std::{
    any::Any,
    borrow::ToOwned,
    convert::TryInto,
    ffi::{c_void, CString},
    fmt::Debug,
    marker::PhantomData,
    mem::{replace, size_of, zeroed, ManuallyDrop},
    os::raw::c_int,
    ptr::null_mut,
    rc::Rc,
    slice,
    sync::atomic::{AtomicPtr, Ordering},
};

/// Predefined interface `Iterator`.
#[inline]
pub fn iterator_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_iterator) }
}

/// Predefined interface `ArrayAccess`.
#[inline]
pub fn array_access_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_arrayaccess) }
}

/// Wrapper of [zend_class_entry].
#[repr(transparent)]
pub struct ClassEntry {
    inner: zend_class_entry,
    _p: PhantomData<*mut ()>,
}

impl ClassEntry {
    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_class_entry) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_class_entry) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_class_entry) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_class_entry) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_class_entry {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_class_entry {
        &mut self.inner
    }

    /// Create reference from global class name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::ClassEntry;
    ///
    /// let std_class = ClassEntry::from_globals("stdClass").unwrap();
    /// let _obj = std_class.new_object([]).unwrap();
    /// ```
    pub fn from_globals(class_name: impl AsRef<str>) -> crate::Result<&'static Self> {
        let name = class_name.as_ref();
        let ptr: *mut Self = find_global_class_entry_ptr(name).cast();
        unsafe {
            ptr.as_ref().ok_or_else(|| {
                crate::Error::ClassNotFound(ClassNotFoundError::new(name.to_string()))
            })
        }
    }

    /// Create the object from class and call `__construct` with arguments.
    ///
    /// If the `__construct` is private, or protected and the called scope isn't
    /// parent class, it will throw PHP Error.
    pub fn new_object(&self, arguments: impl AsMut<[ZVal]>) -> crate::Result<ZObject> {
        let mut object = self.init_object()?;
        object.call_construct(arguments)?;
        Ok(object)
    }

    /// Create the object from class, without calling `__construct`.
    ///
    /// **Be careful when `__construct` is necessary.**
    pub fn init_object(&self) -> crate::Result<ZObject> {
        unsafe {
            let ptr = self.as_ptr() as *mut _;
            let mut val = ZVal::default();
            if !phper_object_init_ex(val.as_mut_ptr(), ptr) {
                Err(InitializeObjectError::new(self.get_name().to_str()?.to_owned()).into())
            } else {
                // Can't drop val here! Otherwise the object will be dropped too (wasting me a
                // day of debugging time here).
                let mut val = ManuallyDrop::new(val);
                let ptr = phper_z_obj_p(val.as_mut_ptr());
                Ok(ZObject::from_raw(ptr))
            }
        }
    }

    /// Get the class name.
    pub fn get_name(&self) -> &ZStr {
        unsafe { ZStr::from_ptr(self.inner.name) }
    }

    /// Detect if the method is exists in class.
    pub fn has_method(&self, method_name: &str) -> bool {
        unsafe {
            let function_table = ZArr::from_ptr(&self.inner.function_table);
            function_table.exists(method_name)
        }
    }

    /// Detect if the class is instance of parent class.
    pub fn is_instance_of(&self, parent: &ClassEntry) -> bool {
        unsafe { phper_instanceof_function(self.as_ptr(), parent.as_ptr()) }
    }

    /// Get the static property by name of class.
    ///
    /// Return None when static property hasn't register by
    /// [ClassEntity::add_static_property].
    pub fn get_static_property(&self, name: impl AsRef<str>) -> Option<&ZVal> {
        let ptr = self.as_ptr() as *mut _;
        let prop = Self::inner_get_static_property(ptr, name);
        unsafe { ZVal::try_from_ptr(prop) }
    }

    /// Set the static property by name of class.
    ///
    /// Return `Some(x)` where `x` is the previous value of static property, or
    /// return `None` when static property hasn't register by
    /// [ClassEntity::add_static_property].
    pub fn set_static_property(&self, name: impl AsRef<str>, val: impl Into<ZVal>) -> Option<ZVal> {
        let ptr = self.as_ptr() as *mut _;
        let prop = Self::inner_get_static_property(ptr, name);
        let prop = unsafe { ZVal::try_from_mut_ptr(prop) };
        prop.map(|prop| replace(prop, val.into()))
    }

    fn inner_get_static_property(scope: *mut zend_class_entry, name: impl AsRef<str>) -> *mut zval {
        let name = name.as_ref();

        unsafe {
            #[allow(clippy::useless_conversion)]
            zend_read_static_property(scope, name.as_ptr().cast(), name.len(), true.into())
        }
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

/// The [StaticStateClass] holds
/// [zend_class_entry] and inner state, always as
/// the static variable, and then be bind to [ClassEntity].
///
/// When the class registered (module initialized), the [StaticStateClass] will
/// be initialized, so you can use the [StaticStateClass] to new stateful
/// object, etc.
///
/// So, You shouldn't use [StaticStateClass] in `module_init` stage, because it
/// hasn't initialized.
///
/// # Examples
///
/// ```rust
/// use phper::classes::{ClassEntity, StaticStateClass};
///
/// pub static FOO_CLASS: StaticStateClass<FooState> = StaticStateClass::null();
///
/// #[derive(Default)]
/// pub struct FooState;
///
/// fn make_foo_class() -> ClassEntity<FooState> {
///     let mut class = ClassEntity::new_with_default_state_constructor("Foo");
///     class.bind(&FOO_CLASS);
///     class
/// }
/// ```
#[repr(transparent)]
pub struct StaticStateClass<T> {
    inner: AtomicPtr<zend_class_entry>,
    _p: PhantomData<T>,
}

impl<T> StaticStateClass<T> {
    /// Create empty [StaticStateClass], with null
    /// [zend_class_entry].
    pub const fn null() -> Self {
        Self {
            inner: AtomicPtr::new(null_mut()),
            _p: PhantomData,
        }
    }

    fn bind(&'static self, ptr: *mut zend_class_entry) {
        self.inner.store(ptr, Ordering::Relaxed);
    }

    /// Converts to class entry.
    pub fn as_class_entry(&'static self) -> &'static ClassEntry {
        unsafe { ClassEntry::from_mut_ptr(self.inner.load(Ordering::Relaxed)) }
    }

    /// Create the object from class and call `__construct` with arguments.
    ///
    /// If the `__construct` is private, or protected and the called scope isn't
    /// parent class, it will throw PHP Error.
    pub fn new_object(
        &'static self, arguments: impl AsMut<[ZVal]>,
    ) -> crate::Result<StateObject<T>> {
        self.as_class_entry()
            .new_object(arguments)
            .map(ZObject::into_raw)
            .map(StateObject::<T>::from_raw_object)
    }

    /// Create the object from class, without calling `__construct`.
    ///
    /// **Be careful when `__construct` is necessary.**
    pub fn init_object(&'static self) -> crate::Result<StateObject<T>> {
        self.as_class_entry()
            .init_object()
            .map(ZObject::into_raw)
            .map(StateObject::<T>::from_raw_object)
    }
}

unsafe impl<T> Sync for StaticStateClass<T> {}

/// The [StaticInterface]  holds
/// [zend_class_entry], always as the static
/// variable, and then be bind to [InterfaceEntity].
///
/// When the interface registered (module initialized), the [StaticInterface]
/// will be initialized.
///
/// So, You shouldn't use [StaticInterface] in `module_init` stage, because it
/// hasn't initialized.
///
/// # Examples
///
/// ```rust
/// use phper::classes::{InterfaceEntity, StaticInterface};
///
/// pub static FOO_INTERFACE: StaticInterface = StaticInterface::null();
///
/// fn make_foo_interface() -> InterfaceEntity {
///     let mut interface = InterfaceEntity::new("Foo");
///     interface.bind(&FOO_INTERFACE);
///     interface
/// }
/// ```
#[repr(transparent)]
pub struct StaticInterface {
    inner: AtomicPtr<zend_class_entry>,
}

impl StaticInterface {
    /// Create empty [StaticInterface], with null
    /// [zend_class_entry].
    pub const fn null() -> Self {
        Self {
            inner: AtomicPtr::new(null_mut()),
        }
    }

    fn bind(&'static self, ptr: *mut zend_class_entry) {
        self.inner.store(ptr, Ordering::Relaxed);
    }

    /// Converts to class entry.
    pub fn as_class_entry(&'static self) -> &'static ClassEntry {
        unsafe { ClassEntry::from_mut_ptr(self.inner.load(Ordering::Relaxed)) }
    }
}

pub(crate) type StateConstructor = dyn Fn() -> *mut dyn Any;

pub(crate) type StateCloner = dyn Fn(*const dyn Any) -> *mut dyn Any;

/// Builder for registering class.
///
/// `<T>` means the type of holding state.
///
/// *It is a common practice for PHP extensions to use PHP objects to package
/// third-party resources.*
pub struct ClassEntity<T: 'static> {
    class_name: CString,
    state_constructor: Rc<StateConstructor>,
    method_entities: Vec<MethodEntity>,
    property_entities: Vec<PropertyEntity>,
    parent: Option<Box<dyn Fn() -> &'static ClassEntry>>,
    interfaces: Vec<Box<dyn Fn() -> &'static ClassEntry>>,
    bind_class: Option<&'static StaticStateClass<T>>,
    state_cloner: Option<Rc<StateCloner>>,
    _p: PhantomData<(*mut (), T)>,
}

impl ClassEntity<()> {
    /// Construct a new `ClassEntity` with class name, do not own state.
    pub fn new(class_name: impl Into<String>) -> Self {
        Self::new_with_state_constructor(class_name, || ())
    }
}

impl<T: Default + 'static> ClassEntity<T> {
    /// Construct a new `ClassEntity` with class name and default state
    /// constructor.
    pub fn new_with_default_state_constructor(class_name: impl Into<String>) -> Self {
        Self::new_with_state_constructor(class_name, Default::default)
    }
}

impl<T: 'static> ClassEntity<T> {
    /// Construct a new `ClassEntity` with class name and the constructor to
    /// build state.
    pub fn new_with_state_constructor(
        class_name: impl Into<String>, state_constructor: impl Fn() -> T + 'static,
    ) -> Self {
        Self {
            class_name: ensure_end_with_zero(class_name),
            state_constructor: Rc::new(move || {
                let state = state_constructor();
                let boxed = Box::new(state) as Box<dyn Any>;
                Box::into_raw(boxed)
            }),
            method_entities: Vec::new(),
            property_entities: Vec::new(),
            parent: None,
            interfaces: Vec::new(),
            bind_class: None,
            state_cloner: None,
            _p: PhantomData,
        }
    }

    /// Add member method to class, with visibility and method handler.
    pub fn add_method<F, Z, E>(
        &mut self, name: impl Into<String>, vis: Visibility, handler: F,
    ) -> &mut MethodEntity
    where
        F: Fn(&mut StateObj<T>, &mut [ZVal]) -> Result<Z, E> + 'static,
        Z: Into<ZVal> + 'static,
        E: Throwable + 'static,
    {
        self.method_entities.push(MethodEntity::new(
            name,
            Some(Rc::new(Method::new(handler))),
            vis,
        ));
        self.method_entities.last_mut().unwrap()
    }

    /// Add static method to class, with visibility and method handler.
    pub fn add_static_method<F, Z, E>(
        &mut self, name: impl Into<String>, vis: Visibility, handler: F,
    ) -> &mut MethodEntity
    where
        F: Fn(&mut [ZVal]) -> Result<Z, E> + 'static,
        Z: Into<ZVal> + 'static,
        E: Throwable + 'static,
    {
        let mut entity = MethodEntity::new(name, Some(Rc::new(Function::new(handler))), vis);
        entity.set_vis_static();
        self.method_entities.push(entity);
        self.method_entities.last_mut().unwrap()
    }

    /// Add abstract method to class, with visibility (shouldn't be private).
    pub fn add_abstract_method(
        &mut self, name: impl Into<String>, vis: Visibility,
    ) -> &mut MethodEntity {
        let mut entity = MethodEntity::new(name, None, vis);
        entity.set_vis_abstract();
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

    /// Declare static property.
    ///
    /// The argument `value` should be `Copy` because 'zend_declare_property'
    /// receive only scalar zval , otherwise will report fatal error:
    /// "Internal zvals cannot be refcounted".
    pub fn add_static_property(
        &mut self, name: impl Into<String>, visibility: Visibility, value: impl Into<Scalar>,
    ) {
        let mut entity = PropertyEntity::new(name, visibility, value);
        entity.set_vis_static();
        self.property_entities.push(entity);
    }

    /// Register class to `extends` the parent class.
    ///
    /// *Because in the `MINIT` phase, the class starts to register, so the*
    /// *closure is used to return the `ClassEntry` to delay the acquisition of*
    /// *the class.*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::{ClassEntity, ClassEntry};
    ///
    /// let mut class = ClassEntity::new("MyException");
    /// class.extends(|| ClassEntry::from_globals("Exception").unwrap());
    /// ```
    pub fn extends(&mut self, parent: impl Fn() -> &'static ClassEntry + 'static) {
        self.parent = Some(Box::new(parent));
    }

    /// Register class to `implements` the interface, due to the class can
    /// implement multi interface, so this method can be called multi time.
    ///
    /// *Because in the `MINIT` phase, the class starts to register, so the*
    /// *closure is used to return the `ClassEntry` to delay the acquisition of*
    /// *the class.*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::{ClassEntity, ClassEntry};
    ///
    /// let mut class = ClassEntity::new("MyClass");
    /// class.implements(|| ClassEntry::from_globals("Stringable").unwrap());
    ///
    /// // Here you have to the implement the method `__toString()` in `Stringable`
    /// // for `MyClass`, otherwise `MyClass` will become abstract class.
    /// // ...
    /// ```
    pub fn implements(&mut self, interface: impl Fn() -> &'static ClassEntry + 'static) {
        self.interfaces.push(Box::new(interface));
    }

    /// Bind to static [StaticStateClass].
    ///
    /// When the class registered, the [StaticStateClass] will be initialized,
    /// so you can use the [StaticStateClass] to new stateful object, etc.
    pub fn bind(&mut self, cls: &'static StaticStateClass<T>) {
        self.bind_class = Some(cls);
    }

    /// Add the state clone function, called when cloning PHP object.
    ///
    /// By default, the object registered by `phper` is uncloneable, if you
    /// clone the object in PHP like this:
    ///
    /// ```php
    /// $foo = new Foo();
    /// $foo2 = clone $foo;
    /// ```
    ///
    /// Will throw the Error: `Uncaught Error: Trying to clone an uncloneable
    /// object of class Foo`.
    ///
    /// And then, if you want the object to be cloneable, you should register
    /// the state clone method for the class.
    ///
    /// # Examples
    ///
    /// ```
    /// use phper::classes::ClassEntity;
    ///
    /// fn make_foo_class() -> ClassEntity<i64> {
    ///     let mut class = ClassEntity::new_with_state_constructor("Foo", || 123456);
    ///     class.state_cloner(Clone::clone);
    ///     class
    /// }
    /// ```
    pub fn state_cloner(&mut self, clone_fn: impl Fn(&T) -> T + 'static) {
        self.state_cloner = Some(Rc::new(move |src| {
            let src = unsafe {
                src.as_ref()
                    .unwrap()
                    .downcast_ref::<T>()
                    .expect("cast Any to T failed")
            };
            let dest = clone_fn(src);
            let boxed = Box::new(dest) as Box<dyn Any>;
            Box::into_raw(boxed)
        }));
    }

    #[allow(clippy::useless_conversion)]
    pub(crate) unsafe fn init(&self) -> *mut zend_class_entry {
        let parent: *mut zend_class_entry = self
            .parent
            .as_ref()
            .map(|parent| parent())
            .map(|entry| entry.as_ptr() as *mut _)
            .unwrap_or(null_mut());

        let class_ce = phper_init_class_entry_ex(
            self.class_name.as_ptr().cast(),
            self.class_name.as_bytes().len().try_into().unwrap(),
            self.function_entries(),
            Some(class_init_handler),
            parent.cast(),
        );

        if let Some(bind_class) = self.bind_class {
            bind_class.bind(class_ce);
        }

        for interface in &self.interfaces {
            let interface_ce = interface().as_ptr();
            zend_class_implements(class_ce, 1, interface_ce);
        }

        *phper_get_create_object(class_ce) = Some(create_object);

        class_ce
    }

    pub(crate) unsafe fn declare_properties(&self, ce: *mut zend_class_entry) {
        for property in &self.property_entities {
            property.declare(ce);
        }
    }

    unsafe fn function_entries(&self) -> *const zend_function_entry {
        let mut methods = self
            .method_entities
            .iter()
            .map(|method| FunctionEntry::from_method_entity(method))
            .collect::<Vec<_>>();

        methods.push(zeroed::<zend_function_entry>());

        // Store the state constructor pointer to zend_class_entry.
        methods.push(self.take_state_constructor_into_function_entry());

        // Store the state cloner pointer to zend_class_entry.
        methods.push(self.take_state_cloner_into_function_entry());

        Box::into_raw(methods.into_boxed_slice()).cast()
    }

    unsafe fn take_state_constructor_into_function_entry(&self) -> zend_function_entry {
        let mut entry = zeroed::<zend_function_entry>();
        let ptr = &mut entry as *mut _ as *mut *const StateConstructor;
        let state_constructor = Rc::into_raw(self.state_constructor.clone());
        ptr.write(state_constructor);
        entry
    }

    unsafe fn take_state_cloner_into_function_entry(&self) -> zend_function_entry {
        let mut entry = zeroed::<zend_function_entry>();
        let ptr = &mut entry as *mut _ as *mut *const StateCloner;
        if let Some(state_cloner) = &self.state_cloner {
            let state_constructor = Rc::into_raw(state_cloner.clone());
            ptr.write(state_constructor);
        }
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

/// Builder for registering interface.
pub struct InterfaceEntity {
    interface_name: CString,
    method_entities: Vec<MethodEntity>,
    extends: Vec<Box<dyn Fn() -> &'static ClassEntry>>,
    bind_interface: Option<&'static StaticInterface>,
}

impl InterfaceEntity {
    /// Construct a new `InterfaceEntity` with interface name.
    pub fn new(interface_name: impl Into<String>) -> Self {
        Self {
            interface_name: ensure_end_with_zero(interface_name.into()),
            method_entities: Vec::new(),
            extends: Vec::new(),
            bind_interface: None,
        }
    }

    /// Add member method to interface, with mandatory visibility public
    /// abstract.
    pub fn add_method(&mut self, name: impl Into<String>) -> &mut MethodEntity {
        let mut entity = MethodEntity::new(name, None, Visibility::Public);
        entity.set_vis_abstract();
        self.method_entities.push(entity);
        self.method_entities.last_mut().unwrap()
    }

    /// Register interface to `extends` the interfaces, due to the interface can
    /// extends multi interface, so this method can be called multi time.
    ///
    /// *Because in the `MINIT` phase, the class starts to register, so the*
    /// *closure is used to return the `ClassEntry` to delay the acquisition of*
    /// *the class.*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::{ClassEntry, InterfaceEntity};
    ///
    /// let mut interface = InterfaceEntity::new("MyInterface");
    /// interface.extends(|| ClassEntry::from_globals("Stringable").unwrap());
    /// ```
    pub fn extends(&mut self, interface: impl Fn() -> &'static ClassEntry + 'static) {
        self.extends.push(Box::new(interface));
    }

    /// Bind to static [StaticInterface].
    ///
    /// When the interface registered, the [StaticInterface] will be
    /// initialized, so you can use the [StaticInterface] to be implemented
    /// by other class.
    pub fn bind(&mut self, i: &'static StaticInterface) {
        self.bind_interface = Some(i);
    }

    #[allow(clippy::useless_conversion)]
    pub(crate) unsafe fn init(&self) -> *mut zend_class_entry {
        let class_ce = phper_init_class_entry_ex(
            self.interface_name.as_ptr().cast(),
            self.interface_name.as_bytes().len().try_into().unwrap(),
            self.function_entries(),
            Some(interface_init_handler),
            null_mut(),
        );

        if let Some(bind_interface) = self.bind_interface {
            bind_interface.bind(class_ce);
        }

        for interface in &self.extends {
            let interface_ce = interface().as_ptr();
            zend_class_implements(class_ce, 1, interface_ce);
        }

        class_ce
    }

    unsafe fn function_entries(&self) -> *const zend_function_entry {
        let mut methods = self
            .method_entities
            .iter()
            .map(|method| FunctionEntry::from_method_entity(method))
            .collect::<Vec<_>>();

        methods.push(zeroed::<zend_function_entry>());

        Box::into_raw(methods.into_boxed_slice()).cast()
    }
}

unsafe extern "C" fn interface_init_handler(
    class_ce: *mut zend_class_entry, _argument: *mut c_void,
) -> *mut zend_class_entry {
    zend_register_internal_interface(class_ce)
}

/// Builder for declare class property.
struct PropertyEntity {
    name: String,
    visibility: RawVisibility,
    value: Scalar,
}

impl PropertyEntity {
    fn new(name: impl Into<String>, visibility: Visibility, value: impl Into<Scalar>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility as RawVisibility,
            value: value.into(),
        }
    }

    #[inline]
    pub(crate) fn set_vis_static(&mut self) -> &mut Self {
        self.visibility |= ZEND_ACC_STATIC;
        self
    }

    #[allow(clippy::useless_conversion)]
    pub(crate) fn declare(&self, ce: *mut zend_class_entry) {
        let name = self.name.as_ptr().cast();
        let name_length = self.name.len().try_into().unwrap();
        let access_type = self.visibility as i32;

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

/// Visibility of class properties and methods.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Visibility {
    /// Public.
    #[default]
    Public = ZEND_ACC_PUBLIC,

    /// Protected.
    Protected = ZEND_ACC_PROTECTED,

    /// Private.
    Private = ZEND_ACC_PRIVATE,
}

/// Raw visibility flag.
pub(crate) type RawVisibility = u32;

#[allow(clippy::useless_conversion)]
unsafe extern "C" fn create_object(ce: *mut zend_class_entry) -> *mut zend_object {
    // Alloc more memory size to store state data.
    let state_object = phper_zend_object_alloc(size_of::<StateObj<()>>().try_into().unwrap(), ce);
    let state_object = StateObj::<()>::from_mut_ptr(state_object);

    // Find the hack elements hidden behind null builtin_function.
    let mut func_ptr = (*ce).info.internal.builtin_functions;
    while !(*func_ptr).fname.is_null() {
        func_ptr = func_ptr.offset(1);
    }

    // Get state constructor.
    func_ptr = func_ptr.offset(1);
    let state_constructor = func_ptr as *mut *const StateConstructor;
    let state_constructor = state_constructor.read().as_ref().unwrap();

    // Get state cloner.
    func_ptr = func_ptr.offset(1);
    let has_state_cloner =
        slice::from_raw_parts(func_ptr as *const u8, size_of::<*const StateCloner>())
            != [0u8; size_of::<*const StateCloner>()];

    // Common initialize process.
    let object = state_object.as_mut_object().as_mut_ptr();
    zend_object_std_init(object, ce);
    object_properties_init(object, ce);
    rebuild_object_properties(object);

    // Set handlers
    let mut handlers = Box::new(std_object_handlers);
    handlers.offset = StateObj::<()>::offset() as c_int;
    handlers.free_obj = Some(free_object);
    handlers.clone_obj = has_state_cloner.then_some(clone_object);
    (*object).handlers = Box::into_raw(handlers);

    // Call the state constructor and store the state.
    let data = (state_constructor)();
    *state_object.as_mut_any_state() = data;

    object
}

#[cfg(phper_major_version = "8")]
unsafe extern "C" fn clone_object(object: *mut zend_object) -> *mut zend_object {
    clone_object_common(object)
}

#[cfg(phper_major_version = "7")]
unsafe extern "C" fn clone_object(object: *mut zval) -> *mut zend_object {
    let object = phper_z_obj_p(object);
    clone_object_common(object)
}

#[allow(clippy::useless_conversion)]
unsafe fn clone_object_common(object: *mut zend_object) -> *mut zend_object {
    let ce = (*object).ce;

    // Alloc more memory size to store state data.
    let new_state_object =
        phper_zend_object_alloc(size_of::<StateObj<()>>().try_into().unwrap(), ce);
    let new_state_object = StateObj::<()>::from_mut_ptr(new_state_object);

    // Find the hack elements hidden behind null builtin_function.
    let mut func_ptr = (*(*object).ce).info.internal.builtin_functions;
    while !(*func_ptr).fname.is_null() {
        func_ptr = func_ptr.offset(1);
    }

    // Get state cloner.
    func_ptr = func_ptr.offset(2);
    let state_cloner = func_ptr as *mut *const StateCloner;
    let state_cloner = state_cloner.read().as_ref().unwrap();

    // Initialize and clone members
    let new_object = new_state_object.as_mut_object().as_mut_ptr();
    zend_object_std_init(new_object, ce);
    object_properties_init(new_object, ce);
    zend_objects_clone_members(new_object, object);

    // Set handlers
    (*new_object).handlers = (*object).handlers;

    // Call the state cloner and store the state.
    let state_object = StateObj::<()>::from_mut_object_ptr(object);
    let data = (state_cloner)(*state_object.as_mut_any_state());
    *new_state_object.as_mut_any_state() = data;

    new_object
}

unsafe extern "C" fn free_object(object: *mut zend_object) {
    let state_object = StateObj::<()>::from_mut_object_ptr(object);

    // Drop the state.
    state_object.drop_state();

    // Original destroy call.
    zend_object_std_dtor(object);
}
