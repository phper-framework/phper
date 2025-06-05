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
    alloc::EBox,
    classes::{
        ClassEntry, ConstantEntity, InnerClassEntry, Interface, Visibility, add_class_constant,
    },
    errors::Throwable,
    functions::{Function, FunctionEntry, HandlerMap, MethodEntity},
    objects::ZObj,
    strings::ZStr,
    sys::*,
    types::Scalar,
    utils::ensure_end_with_zero,
    values::ZVal,
};
use sealed::sealed;
use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::{ManuallyDrop, zeroed},
    ptr::{null, null_mut},
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
struct EnumCaseEntity {
    name: CString,
    value: Scalar,
}

/// Represents an enum case within a PHP enum.
///
/// `EnumCase` provides a convenient way to access a specific enum case
/// without repeatedly calling `Enum::get_case()` with the case name.
/// It stores a reference to the enum and the name of the case.
#[derive(Clone)]
pub struct EnumCase {
    r#enum: Enum,
    case_name: String,
}

impl EnumCase {
    /// Creates a new `EnumCase` with the specified enum and case name.
    ///
    /// # Parameters
    ///
    /// * `enum_obj` - The enum containing the case
    /// * `case_name` - The name of the enum case
    fn new(enum_obj: Enum, case_name: impl Into<String>) -> Self {
        Self {
            r#enum: enum_obj,
            case_name: case_name.into(),
        }
    }

    /// Gets a reference to the enum case.
    ///
    /// # Returns
    ///
    /// A reference to ZObj representing the enum case, or an error if the case
    /// doesn't exist
    pub fn get_case<'a>(&self) -> &'a ZObj {
        unsafe { self.r#enum.get_case(&self.case_name).unwrap() }
    }

    /// Gets a mutable reference to the enum case.
    ///
    /// # Returns
    ///
    /// A mutable reference to ZObj representing the enum case, or an error if
    /// the case doesn't exist
    pub fn get_mut_case<'a>(&mut self) -> &'a mut ZObj {
        unsafe { self.r#enum.get_mut_case(&self.case_name).unwrap() }
    }

    /// Gets the name of the enum case.
    pub fn name(&self) -> &str {
        &self.case_name
    }

    /// Gets the enum this case belongs to.
    pub fn as_enum(&self) -> &Enum {
        &self.r#enum
    }
}

/// The [Enum] holds [zend_class_entry] for PHP enum, created by
/// [Module::add_enum](crate::modules::Module::add_enum) or
/// [EnumEntity::bound_enum].
///
/// When the enum registered (module initialized), the [Enum] will
/// be initialized, so you can use the [Enum] to get enum cases, etc.
///
/// # Examples
///
/// ```rust
/// use phper::{
///     enums::{Enum, EnumEntity},
///     modules::Module,
///     php_get_module,
/// };
///
/// fn make_status_enum() -> EnumEntity {
///     let mut enum_entity = EnumEntity::new("Status");
///     enum_entity.add_case("Active", ());
///     enum_entity.add_case("Inactive", ());
///     enum_entity.add_case("Pending", ());
///     enum_entity
/// }
///
/// #[php_get_module]
/// pub fn get_module() -> Module {
///     let mut module = Module::new(
///         env!("CARGO_CRATE_NAME"),
///         env!("CARGO_PKG_VERSION"),
///         env!("CARGO_PKG_AUTHORS"),
///     );
///
///     let _status_enum: Enum = module.add_enum(make_status_enum());
///
///     module
/// }
/// ```
#[derive(Clone)]
pub struct Enum {
    inner: Rc<RefCell<InnerClassEntry>>,
}

impl Enum {
    /// Creates a null Enum reference. Used internally.
    fn null() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerClassEntry::Ptr(null()))),
        }
    }

    /// Create from name, which will be looked up from globals.
    pub fn from_name(name: impl Into<String>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerClassEntry::Name(name.into()))),
        }
    }

    fn bind(&self, ptr: *mut zend_class_entry) {
        match &mut *self.inner.borrow_mut() {
            InnerClassEntry::Ptr(p) => {
                *p = ptr;
            }
            InnerClassEntry::Name(_) => {
                unreachable!("Cannot bind() an Enum created with from_name()");
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

    /// Get an enum case by name.
    ///
    /// # Parameters
    ///
    /// * `case_name` - The name of the enum case to retrieve
    ///
    /// # Returns
    ///
    /// A reference to ZObj representing the enum case, or an error if the case
    /// doesn't exist
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because the underlying
    /// `zend_enum_get_case` function may cause a SIGSEGV (segmentation
    /// fault) when the case doesn't exist. Even though this method attempts
    /// to check for null and return an error instead, there might still be
    /// scenarios where the PHP internal function behaves unpredictably.
    /// Callers must ensure the enum and case name are valid before calling this
    /// function.
    pub unsafe fn get_case<'a>(&self, case_name: impl AsRef<str>) -> crate::Result<&'a ZObj> {
        unsafe {
            let ce = self.as_class_entry().as_ptr() as *mut _;
            let case_name_str = case_name.as_ref();
            let mut name_zstr = EBox::<ZStr>::new(case_name_str);

            // Get the enum case
            let case_obj = zend_enum_get_case(ce, name_zstr.as_mut_ptr());

            // Convert to &ZObj
            Ok(ZObj::from_ptr(case_obj))
        }
    }

    /// Get a mutable reference to an enum case by name.
    ///
    /// # Parameters
    ///
    /// * `case_name` - The name of the enum case to retrieve
    ///
    /// # Returns
    ///
    /// A mutable reference to ZObj representing the enum case, or an error if
    /// the case doesn't exist
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because the underlying
    /// `zend_enum_get_case` function may cause a SIGSEGV (segmentation
    /// fault) when the case doesn't exist. Even though this method attempts
    /// to check for null and return an error instead, there might still be
    /// scenarios where the PHP internal function behaves unpredictably.
    /// Callers must ensure the enum and case name are valid before calling this
    /// function.
    pub unsafe fn get_mut_case<'a>(
        &mut self, case_name: impl AsRef<str>,
    ) -> crate::Result<&'a mut ZObj> {
        unsafe {
            let ce = self.as_class_entry().as_ptr() as *mut _;
            let case_name_str = case_name.as_ref();
            let mut name_zstr = EBox::<ZStr>::new(case_name_str);

            // Get the enum case
            let case_obj = zend_enum_get_case(ce, name_zstr.as_mut_ptr());

            // Convert to &mut ZObj
            Ok(ZObj::from_mut_ptr(case_obj as *mut _))
        }
    }
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
    cases: Vec<EnumCaseEntity>,
    constants: Vec<ConstantEntity>,
    interfaces: Vec<Interface>,
    bound_enum: Enum,
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
            bound_enum: Enum::null(),
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
    /// An `EnumCase` instance representing the added case
    pub fn add_case(&mut self, name: impl Into<String>, value: B) -> EnumCase {
        let case_name_str = name.into();
        let case_name = ensure_end_with_zero(&case_name_str);
        self.cases.push(EnumCaseEntity {
            name: case_name,
            value: value.into(),
        });
        EnumCase::new(self.bound_enum(), case_name_str)
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

    /// Get the bound enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use phper::{alloc::ToRefOwned, classes::Visibility, enums::EnumEntity};
    ///
    /// pub fn make_status_enum() -> EnumEntity {
    ///     let mut enum_entity = EnumEntity::new("Status");
    ///     enum_entity.add_case("Active", ());
    ///     enum_entity.add_case("Inactive", ());
    ///     let mut status_enum = enum_entity.bound_enum();
    ///     enum_entity.add_static_method("getActiveCase", Visibility::Public, move |_| {
    ///         let active_case = unsafe { status_enum.clone().get_mut_case("Active")? };
    ///         phper::ok(active_case.to_ref_owned())
    ///     });
    ///     enum_entity
    /// }
    /// ```
    #[inline]
    pub fn bound_enum(&self) -> Enum {
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
                let value = EBox::<ZStr>::new_persistent(value);
                let mut value = ManuallyDrop::new(ZVal::from(value));
                zend_enum_add_case_cstr(class_ce, case_name.as_ptr(), value.as_mut_ptr());
            }
            Scalar::Null => {
                zend_enum_add_case_cstr(class_ce, case_name.as_ptr(), null_mut());
            }
            _ => unreachable!(),
        };
    }
}
