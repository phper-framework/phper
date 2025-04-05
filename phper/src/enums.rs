// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! APIs related to PHP enum functionality.
//!
//! This module provides Rust wrappers for PHP enum functionality, allowing you
//! to define and work with PHP enums from Rust code. It supports pure enums,
//! integer-backed enums, and string-backed enums, corresponding to their PHP
//! counterparts.
//!
//! The implementation respects the PHP 8.1+ enum feature set and provides a
//! type-safe interface for creating enum cases and handling enum values.
#![cfg(phper_enum_supported)]

use crate::{
    classes::{ConstantEntity, Interface, Visibility, add_class_constant},
    errors::Throwable,
    functions::{Function, FunctionEntry, HandlerMap, MethodEntity},
    sys::*,
    types::Scalar,
    utils::ensure_end_with_zero,
    values::ZVal,
};
use sealed::sealed;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::zeroed,
    ptr::null_mut,
    rc::Rc,
};

/// Trait representing a backing type for enum values.
///
/// This trait is implemented by types that can serve as backing values
/// for PHP enums. The trait is sealed to ensure only supported types
/// can be used as enum backing types.
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

/// Enum type in PHP.
///
/// Represents the three possible types of PHP enums:
/// - Pure enums (no backing value)
/// - Integer-backed enums
/// - String-backed enums
pub enum EnumType {
    /// Pure enum (like `enum Foo { case A, case B }`)
    Pure,
    /// Int backed enum (like `enum Foo: int { case A = 1, case B = 2 }`)
    IntBacked,
    /// String backed enum (like `enum Foo: string { case A = 'a', case B = 'b'
    /// }`)
    StringBacked,
}

/// Enum case definition for PHP enum.
///
/// Represents a single case within a PHP enum, storing its name
/// and associated value.
struct EnumCase {
    name: CString,
    value: Scalar,
}

/// Builder for registering a PHP enum.
///
/// This struct facilitates the creation and registration of PHP enums from Rust
/// code. The generic parameter B represents the backing type and determines the
/// enum type.
///
/// # Type Parameters
///
/// * `B` - A type that implements `EnumBackingType`, determining the enum's
///   backing type. Use `()` for pure enums, `i64` for int-backed enums, or
///   `String` for string-backed enums.
pub struct EnumEntity<B: EnumBackingType = ()> {
    enum_name: CString,
    enum_type: EnumType,
    method_entities: Vec<MethodEntity>,
    cases: Vec<EnumCase>,
    constants: Vec<ConstantEntity>,
    interfaces: Vec<Interface>,
    _p: PhantomData<(B, *mut ())>,
}

impl<B: EnumBackingType> EnumEntity<B> {
    /// Creates a new enum builder with the specified name.
    ///
    /// # Parameters
    ///
    /// * `enum_name` - The name of the PHP enum to create
    ///
    /// # Returns
    ///
    /// A new `EnumEntity` instance configured for the specified enum type
    pub fn new(enum_name: impl Into<String>) -> Self {
        Self {
            enum_name: ensure_end_with_zero(enum_name),
            enum_type: B::enum_type(),
            method_entities: Vec::new(),
            cases: Vec::new(),
            constants: Vec::new(),
            interfaces: Vec::new(),
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
    pub fn add_case(&mut self, name: impl Into<String>, value: B) {
        let case_name = ensure_end_with_zero(name);
        self.cases.push(EnumCase {
            name: case_name.clone(),
            value: value.into(),
        });
    }

    /// Adds a static method to the enum.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the method
    /// * `vis` - The visibility of the method (public, protected, or private)
    /// * `handler` - The function that implements the method logic
    ///
    /// # Returns
    ///
    /// A mutable reference to the created `MethodEntity` for further
    /// configuration
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

    /// Adds a constant to the enum.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the constant
    /// * `value` - The value of the constant, which will be converted to a
    ///   Scalar
    pub fn add_constant(&mut self, name: impl Into<String>, value: impl Into<Scalar>) {
        let constant = ConstantEntity::new(name, value);
        self.constants.push(constant);
    }

    /// Registers the enum to implement the specified interface.
    ///
    /// # Parameters
    ///
    /// * `interface` - The interface that the enum should implement
    pub fn implements(&mut self, interface: Interface) {
        self.interfaces.push(interface);
    }

    unsafe fn function_entries(&self) -> *const zend_function_entry {
        unsafe {
            let mut methods = self
                .method_entities
                .iter()
                .map(|method| FunctionEntry::from_method_entity(method))
                .collect::<Vec<_>>();

            methods.push(zeroed::<zend_function_entry>());

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

            class_ce
        }
    }
}

/// Helper function to register an enum case with the PHP engine.
///
/// # Parameters
///
/// * `class_ce` - Pointer to the class entry
/// * `case_name` - Name of the enum case
/// * `case_value` - Value associated with the case
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
                #[allow(clippy::useless_conversion)]
                let value_ptr = phper_zend_string_init(
                    value.as_ptr().cast(),
                    value.len().try_into().unwrap(),
                    true.into(),
                );
                let mut value = ZVal::from(());
                phper_zval_str(value.as_mut_ptr(), value_ptr);

                zend_enum_add_case_cstr(class_ce, case_name.as_ptr(), value.as_mut_ptr());
            }
            Scalar::Null => {
                zend_enum_add_case_cstr(class_ce, case_name.as_ptr(), null_mut());
            }
            _ => unreachable!(),
        };
    }
}
