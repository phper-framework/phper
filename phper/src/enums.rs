// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to PHP enum.
#![cfg(phper_enum_supported)]

use crate::{
    classes::{
        ClassEntry, ConstantEntity, InnerClassEntry, Interface, StateConstructor, Visibility,
        add_class_constant, create_object,
    },
    errors::Throwable,
    functions::{FunctionEntry, HandlerMap, Method, MethodEntity},
    objects::StateObj,
    sys::*,
    types::Scalar,
    utils::ensure_end_with_zero,
    values::ZVal,
};
use sealed::sealed;
use std::{
    any::Any,
    cell::RefCell,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::zeroed,
    ptr::{null, null_mut},
    rc::Rc,
};

/// Trait representing a backing type for enum values.
#[sealed]
pub trait EnumBackingType: Into<Scalar> {
    /// Returns the PHP enum type representation for this backing type.
    fn enum_type() -> EnumType;
}

#[sealed]
impl EnumBackingType for () {
    fn enum_type() -> EnumType {
        EnumType::Pure
    }
}

#[sealed]
impl EnumBackingType for i64 {
    fn enum_type() -> EnumType {
        EnumType::IntBacked
    }
}

#[sealed]
impl EnumBackingType for String {
    fn enum_type() -> EnumType {
        EnumType::StringBacked
    }
}

/// Enum type in PHP
pub enum EnumType {
    /// Pure enum (like `enum Foo { case A, case B }`)
    Pure,
    /// Int backed enum (like `enum Foo: int { case A = 1, case B = 2 }`)
    IntBacked,
    /// String backed enum (like `enum Foo: string { case A = 'a', case B = 'b'
    /// }`)
    StringBacked,
}

/// Enum case definition for PHP enum
struct EnumCase {
    name: CString,
    value: Scalar,
}

/// Reference to a specific enum case
pub struct StateEnumCase<T> {
    enum_name: CString,
    case_name: CString,
    bound_enum: StateEnum<T>,
    _p: PhantomData<T>,
}

impl<T> StateEnumCase<T> {
    fn new(enum_name: CString, case_name: CString, bound_enum: StateEnum<T>) -> Self {
        Self {
            enum_name,
            case_name,
            bound_enum,
            _p: PhantomData,
        }
    }

    /// Gets the name of this enum case
    pub fn name(&self) -> &CStr {
        &self.case_name
    }

    /// Gets the StateEnum this case belongs to
    pub fn enum_type(&self) -> StateEnum<T> {
        self.bound_enum.clone()
    }

    /// Gets the corresponding enum case object instance
    ///
    /// This requires the enum to be fully registered in PHP.
    pub fn get_case_object(&self) -> crate::Result<&StateObj<T>> {
        // Get the class entry for the enum
        let ce = self.bound_enum.as_class_entry();

        unsafe {
            // Find the case in the enum
            let case_zval = zend_enum_get_case_cstr(ce.as_ptr() as *mut _, self.case_name.as_ptr());

            if case_zval.is_null() {
                return Err(crate::Error::boxed(format!(
                    "Enum case {} not found in enum {}",
                    self.case_name.to_string_lossy(),
                    self.enum_name.to_string_lossy()
                )));
            }

            // Convert to StateObj
            Ok(StateObj::<T>::from_object_ptr(phper_z_obj_p(
                case_zval as *const _,
            )))
        }
    }

    /// Gets the corresponding enum case object instance
    ///
    /// This requires the enum to be fully registered in PHP.
    pub fn get_mut_case_object(&mut self) -> crate::Result<&mut StateObj<T>> {
        // Get the class entry for the enum
        let ce = self.bound_enum.as_class_entry();

        unsafe {
            // Find the case in the enum
            let case_zval = zend_enum_get_case_cstr(ce.as_ptr() as *mut _, self.case_name.as_ptr());

            if case_zval.is_null() {
                return Err(crate::Error::boxed(format!(
                    "Enum case {} not found in enum {}",
                    self.case_name.to_string_lossy(),
                    self.enum_name.to_string_lossy()
                )));
            }

            // Convert to StateObj
            Ok(StateObj::<T>::from_mut_object_ptr(phper_z_obj_p(
                case_zval as *const _,
            )))
        }
    }
}

impl<T> Clone for StateEnumCase<T> {
    fn clone(&self) -> Self {
        Self {
            enum_name: self.enum_name.clone(),
            case_name: self.case_name.clone(),
            bound_enum: self.bound_enum.clone(),
            _p: PhantomData,
        }
    }
}

/// Builder for registering an enum.
/// B is a backing type that implements EnumBackingType
pub struct EnumEntity<B: EnumBackingType, T: ?Sized> {
    enum_name: CString,
    enum_type: EnumType,
    method_entities: Vec<MethodEntity>,
    cases: Vec<EnumCase>,
    constants: Vec<ConstantEntity>,
    interfaces: Vec<Interface>,
    bound_enum: StateEnum<T>,
    state_constructor: Rc<StateConstructor>,
    _p: PhantomData<(B, *mut ())>,
}

// Simplified constructor methods
impl<B: EnumBackingType> EnumEntity<B, ()> {
    /// General constructor, automatically determines enum type based on generic
    /// B
    pub fn new(enum_name: impl Into<String>) -> Self {
        Self::new_with_state_constructor(enum_name, || ())
    }
}

impl<B: EnumBackingType, T: 'static> EnumEntity<B, T> {
    /// Creates an enum entity with a custom state constructor.
    ///
    /// This constructor allows creation of enums with associated state data
    /// that will be instantiated for each enum instance through the
    /// provided state constructor function.
    ///
    /// # Parameters
    ///
    /// * `enum_name` - The name of the enum to register in PHP
    /// * `state_constructor` - Function that creates the initial state for enum
    ///   instances
    ///
    /// # Returns
    ///
    /// Returns a new `EnumEntity` instance configured with the provided state
    /// constructor
    ///
    /// # Example
    ///
    /// ```
    /// # use phper::enums::EnumEntity;
    /// struct MyState {
    ///     counter: i32,
    /// }
    ///
    /// let enum_entity =
    ///     EnumEntity::<(), MyState>::new_with_state_constructor("MyEnum", || MyState { counter: 0 });
    /// ```
    pub fn new_with_state_constructor(
        enum_name: impl Into<String>, state_constructor: impl Fn() -> T + 'static,
    ) -> Self {
        Self {
            enum_name: ensure_end_with_zero(enum_name),
            enum_type: B::enum_type(),
            method_entities: Vec::new(),
            cases: Vec::new(),
            constants: Vec::new(),
            interfaces: Vec::new(),
            bound_enum: StateEnum::null(),
            state_constructor: Rc::new(move || {
                let state = state_constructor();
                let boxed = Box::new(state) as Box<dyn Any>;
                Box::into_raw(boxed)
            }),
            _p: PhantomData,
        }
    }

    /// Add a case to the enum with the given name and value.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the enum case
    /// * `value` - The value associated with the enum case, type determined by
    ///   backing type B
    ///
    /// # Returns
    ///
    /// Returns a reference to the created enum case
    pub fn add_case(&mut self, name: impl Into<String>, value: B) -> StateEnumCase<T> {
        let case_name = ensure_end_with_zero(name);
        self.cases.push(EnumCase {
            name: case_name.clone(),
            value: value.into(),
        });
        StateEnumCase::new(self.enum_name.clone(), case_name, self.bound_enum.clone())
    }

    /// Add member method to enum that can access the enum state.
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

    /// Add constant to enum
    pub fn add_constant(&mut self, name: impl Into<String>, value: impl Into<Scalar>) {
        let constant = ConstantEntity::new(name, value);
        self.constants.push(constant);
    }

    /// Register enum to `implements` the interface
    pub fn implements(&mut self, interface: Interface) {
        self.interfaces.push(interface);
    }

    /// Get the bound enum.
    #[inline]
    pub fn bound_enum(&self) -> StateEnum<T> {
        self.bound_enum.clone()
    }

    unsafe fn function_entries(&self) -> *const zend_function_entry {
        unsafe {
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
    }

    pub(crate) fn handler_map(&self) -> HandlerMap {
        self.method_entities
            .iter()
            .filter_map(|method| {
                method.handler.as_ref().map(|handler| {
                    (
                        (Some(self.enum_name.clone()), method.name.clone()),
                        handler.clone(),
                    )
                })
            })
            .collect()
    }

    #[allow(clippy::useless_conversion)]
    pub(crate) unsafe fn init(&self) -> *mut zend_class_entry {
        unsafe {
            let backing_type = match self.enum_type {
                EnumType::Pure => IS_NULL,
                EnumType::IntBacked => IS_LONG,
                EnumType::StringBacked => IS_STRING,
            } as u8;

            let class_ce = zend_register_internal_enum(
                self.enum_name.as_ptr().cast(),
                backing_type,
                self.function_entries(),
            );

            self.bound_enum.bind(class_ce);

            for interface in &self.interfaces {
                let interface_ce = interface.as_class_entry().as_ptr();
                zend_class_implements(class_ce, 1, interface_ce);
            }

            for constant in &self.constants {
                add_class_constant(class_ce, constant);
            }

            // Register all enum cases
            for case in &self.cases {
                register_enum_case(class_ce, &case.name, &case.value);
            }

            *phper_get_create_object(class_ce) = Some(create_object);

            class_ce
        }
    }

    unsafe fn take_state_constructor_into_function_entry(&self) -> zend_function_entry {
        unsafe {
            let mut entry = zeroed::<zend_function_entry>();
            let ptr = &mut entry as *mut _ as *mut *const StateConstructor;
            let state_constructor = Rc::into_raw(self.state_constructor.clone());
            ptr.write(state_constructor);
            entry
        }
    }

    unsafe fn take_state_cloner_into_function_entry(&self) -> zend_function_entry {
        unsafe { zeroed::<zend_function_entry>() }
    }
}

/// Wrapper for the PHP enum that holds state
pub struct StateEnum<T: ?Sized> {
    inner: Rc<RefCell<InnerClassEntry>>,
    _p: PhantomData<T>,
}

impl<T: ?Sized> StateEnum<T> {
    fn null() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerClassEntry::Ptr(null()))),
            _p: PhantomData,
        }
    }

    /// Create from name, which will be looked up from globals.
    pub fn from_name(name: impl Into<String>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerClassEntry::Name(name.into()))),
            _p: PhantomData,
        }
    }

    fn bind(&self, ptr: *mut zend_class_entry) {
        match &mut *self.inner.borrow_mut() {
            InnerClassEntry::Ptr(p) => {
                *p = ptr;
            }
            InnerClassEntry::Name(_) => {
                unreachable!("Cannot bind() an StateEnum created with from_name()");
            }
        }
    }

    /// Converts to class entry.
    pub fn as_class_entry(&self) -> &ClassEntry {
        let inner = self.inner.borrow().clone();
        match inner {
            InnerClassEntry::Ptr(ptr) => unsafe { ClassEntry::from_ptr(ptr) },
            InnerClassEntry::Name(name) => {
                let entry = ClassEntry::from_globals(name).unwrap();
                *self.inner.borrow_mut() = InnerClassEntry::Ptr(entry.as_ptr());
                entry
            }
        }
    }
}

impl<T> Clone for StateEnum<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _p: self._p,
        }
    }
}

// Helper function to register enum case
unsafe fn register_enum_case(
    class_ce: *mut zend_class_entry, case_name: &CStr, case_value: &Scalar,
) {
    unsafe {
        match case_value {
            Scalar::I64(value) => {
                zend_enum_add_case_cstr(
                    class_ce,
                    case_name.as_ptr(),
                    ZVal::from(*value).as_mut_ptr(),
                );
            }
            Scalar::String(value) => {
                zend_enum_add_case_cstr(
                    class_ce,
                    case_name.as_ptr(),
                    ZVal::from(value.clone()).as_mut_ptr(),
                );
            }
            Scalar::Null => {
                zend_enum_add_case_cstr(class_ce, case_name.as_ptr(), null_mut());
            }
            _ => unreachable!(),
        };
    }
}
