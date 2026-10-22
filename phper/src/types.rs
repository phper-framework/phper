// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to PHP types.

use crate::sys::*;
use derive_more::From;
#[cfg(phper_major_version = "8")]
use std::ffi::CString;
use std::{
    ffi::CStr,
    fmt::{self, Debug, Display},
    os::raw::c_int,
};

/// Wrapper of PHP type.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TypeInfo {
    t: u32,
}

impl TypeInfo {
    /// Array type info.
    pub const ARRAY: TypeInfo = TypeInfo::from_raw(IS_ARRAY);
    /// Boolean type info.
    pub const BOOL: TypeInfo = TypeInfo::from_raw(_IS_BOOL);
    /// Double number type info.
    pub const DOUBLE: TypeInfo = TypeInfo::from_raw(IS_DOUBLE);
    /// Long number type info.
    pub const LONG: TypeInfo = TypeInfo::from_raw(IS_LONG);
    /// Null type info.
    pub const NULL: TypeInfo = TypeInfo::from_raw(IS_NULL);
    /// Object type info.
    pub const OBJECT: TypeInfo = TypeInfo::from_raw(IS_OBJECT);
    /// Reference type info.
    pub const REFERENCE: TypeInfo = TypeInfo::from_raw(IS_REFERENCE);
    /// Resource type info.
    pub const RESOURCE: TypeInfo = TypeInfo::from_raw(IS_RESOURCE);
    /// String type info.
    pub const STRING: TypeInfo = TypeInfo::from_raw(IS_STRING);
    /// Undefined type info.
    pub const UNDEF: TypeInfo = TypeInfo::from_raw(IS_UNDEF);
}

impl TypeInfo {
    /// Construct [`TypeInfo`] from raw number.
    pub const fn from_raw(t: u32) -> Self {
        Self { t }
    }

    /// Transfers [`TypeInfo`] to raw number.
    pub const fn into_raw(self) -> u32 {
        self.t
    }

    /// Detects if the [`TypeInfo`] is undefined.
    pub const fn is_undef(self) -> bool {
        self.t == IS_UNDEF
    }

    /// Detects if the [`TypeInfo`] is null.
    pub const fn is_null(self) -> bool {
        self.t == IS_NULL
    }

    /// Detects if the [`TypeInfo`] is boolean.
    pub const fn is_bool(self) -> bool {
        self.is_true() || self.is_false()
    }

    /// Detects if the [`TypeInfo`] is true.
    pub const fn is_true(self) -> bool {
        get_base_type_by_raw(self.t) == IS_TRUE
    }

    /// Detects if the [`TypeInfo`] is false.
    pub const fn is_false(self) -> bool {
        get_base_type_by_raw(self.t) == IS_FALSE
    }

    /// Detects if the [`TypeInfo`] is long.
    pub const fn is_long(self) -> bool {
        get_base_type_by_raw(self.t) == IS_LONG
    }

    /// Detects if the [`TypeInfo`] is double.
    pub const fn is_double(self) -> bool {
        get_base_type_by_raw(self.t) == IS_DOUBLE
    }

    /// Detects if the [`TypeInfo`] is string.
    pub const fn is_string(self) -> bool {
        get_base_type_by_raw(self.t) == IS_STRING
    }

    /// Detects if the [`TypeInfo`] is array.
    pub const fn is_array(self) -> bool {
        get_base_type_by_raw(self.t) == IS_ARRAY
    }

    /// Detects if the [`TypeInfo`] is object.
    pub const fn is_object(self) -> bool {
        get_base_type_by_raw(self.t) == IS_OBJECT
    }

    /// Detects if the [`TypeInfo`] is resource.
    pub const fn is_resource(self) -> bool {
        get_base_type_by_raw(self.t) == IS_RESOURCE
    }

    /// Detects if the [`TypeInfo`] is reference.
    pub const fn is_reference(self) -> bool {
        get_base_type_by_raw(self.t) == IS_REFERENCE
    }

    /// Transfers [`TypeInfo`] to the base type info.
    pub const fn get_base_type(self) -> TypeInfo {
        Self::from_raw(get_base_type_by_raw(self.t))
    }

    /// Gets the name of base type.
    ///
    /// Can be sure, `zend_get_type_by_const` returns the const string, So this
    /// method returns `&'static CStr`.
    #[inline]
    pub fn get_base_type_name(self) -> &'static CStr {
        unsafe {
            let t = get_base_type_by_raw(self.t);

            if t == IS_UNDEF {
                return c"undef";
            }
            if t == IS_REFERENCE {
                return c"reference";
            }

            let s = zend_get_type_by_const(t as c_int);
            let s = CStr::from_ptr(s);

            // Compact with PHP7.
            let bs = s.to_bytes();
            if bs == b"boolean" {
                return c"bool";
            }
            if bs == b"integer" {
                return c"int";
            }

            s
        }
    }
}

impl Debug for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypeInfo")
            .field("base_name", &self.get_base_type_name())
            .field("base", &self.get_base_type().t)
            .field("raw", &self.t)
            .finish()
    }
}

impl From<u32> for TypeInfo {
    fn from(n: u32) -> Self {
        Self::from_raw(n)
    }
}

impl Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self.get_base_type_name().to_str().unwrap_or("unknown");
        Display::fmt(t, f)
    }
}

const fn get_base_type_by_raw(t: u32) -> u32 {
    t & !(!0 << Z_TYPE_FLAGS_SHIFT)
}

/// Copyable value, used in constant and class property.
#[derive(From)]
pub enum Scalar {
    /// Null.
    Null,
    /// Boolean.
    Bool(bool),
    /// Long.
    I64(i64),
    /// Double.
    F64(f64),
    /// String
    String(String),
    /// Binary string.
    Bytes(Vec<u8>),
}

impl From<()> for Scalar {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<&str> for Scalar {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<&[u8]> for Scalar {
    fn from(b: &[u8]) -> Self {
        Self::Bytes(b.to_owned())
    }
}

/// PHP argument typehints
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArgumentTypeHint {
    /// null typehint
    Null,
    /// bool typehint
    Bool,
    /// int typehint
    Int,
    /// float typehint
    Float,
    /// string typehint
    String,
    /// array typehint
    Array,
    /// object typehint
    Object,
    /// callable typehint
    Callable,
    /// iterable typehint
    Iterable,
    /// mixed typehint (php 8.0+)
    Mixed,
    /// ClassEntry typehint (class, interface)
    ClassEntry(String),
    /// false typehint (php 8.0+)
    False,
    /// union of two or more typehints, such as `Foo|false|null` (php 8.0+)
    Union(Vec<ArgumentTypeHint>),
}

impl ArgumentTypeHint {
    /// Returns the Zend type constant for this type hint, if it maps to a
    /// simple constant.
    pub(crate) fn zend_type_const(&self) -> Option<u32> {
        match self {
            Self::Null => Some(IS_NULL),
            Self::Bool => Some(_IS_BOOL),
            Self::Int => Some(IS_LONG),
            Self::Float => Some(IS_DOUBLE),
            Self::String => Some(IS_STRING),
            Self::Array => Some(IS_ARRAY),
            Self::Object => Some(IS_OBJECT),
            Self::Callable => Some(IS_CALLABLE),
            Self::Iterable => Some(IS_ITERABLE),
            Self::Mixed => Some(IS_MIXED),
            Self::ClassEntry(_) | Self::False | Self::Union(_) => None,
        }
    }

    /// Returns the members of a checked union, or of a standalone false.
    pub(crate) fn union_members(&self) -> Result<Vec<UnionMember<'_>>, String> {
        match self {
            Self::Union(members) => {
                let members = members.iter().map(Self::union_member).collect::<Vec<_>>();
                check_union(&members)?;
                Ok(members)
            }
            hint => Ok(vec![hint.union_member()]),
        }
    }

    fn union_member(&self) -> UnionMember<'_> {
        match self {
            Self::Null => UnionMember::Null,
            Self::False => UnionMember::False,
            Self::Bool => UnionMember::Bool,
            Self::Int => UnionMember::Int,
            Self::Float => UnionMember::Float,
            Self::String => UnionMember::String,
            Self::Array => UnionMember::Array,
            Self::Object => UnionMember::Object,
            Self::Callable => UnionMember::Callable,
            Self::Iterable => UnionMember::Iterable,
            Self::ClassEntry(class_name) => UnionMember::Class(class_name),
            Self::Mixed => UnionMember::Standalone("mixed"),
            Self::Union(_) => UnionMember::Union,
        }
    }
}

/// PHP property typehints (php 8.0+)
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PropertyTypeHint {
    /// bool typehint
    Bool,
    /// int typehint
    Int,
    /// float typehint
    Float,
    /// string typehint
    String,
    /// array typehint
    Array,
    /// object typehint
    Object,
    /// iterable typehint
    Iterable,
    /// mixed typehint
    Mixed,
    /// ClassEntry typehint (class, interface)
    ClassEntry(String),
}

impl PropertyTypeHint {
    /// Returns the Zend type constant for this type hint, if it maps to a
    /// simple constant.
    pub(crate) fn zend_type_const(&self) -> Option<u32> {
        match self {
            Self::Bool => Some(_IS_BOOL),
            Self::Int => Some(IS_LONG),
            Self::Float => Some(IS_DOUBLE),
            Self::String => Some(IS_STRING),
            Self::Array => Some(IS_ARRAY),
            Self::Object => Some(IS_OBJECT),
            Self::Iterable => Some(IS_ITERABLE),
            Self::Mixed => Some(IS_MIXED),
            Self::ClassEntry(_) => None,
        }
    }
}

/// PHP return typehints
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReturnTypeHint {
    /// null typehint
    Null,
    /// bool typehint
    Bool,
    /// int typehint
    Int,
    /// float typehint
    Float,
    /// string typehint
    String,
    /// array typehint
    Array,
    /// object typehint
    Object,
    /// callable typehint
    Callable,
    /// iterable typehint
    Iterable,
    /// mixed typehint (php 8.0+)
    Mixed,
    /// ClassEntry typehint (class, interface)
    ClassEntry(String),
    /// never typehint (8.1+)
    Never,
    /// void typehint
    Void,
    /// false typehint (php 8.0+)
    False,
    /// union of two or more typehints, such as `Foo|false` (php 8.0+)
    Union(Vec<ReturnTypeHint>),
}

impl ReturnTypeHint {
    /// Returns the Zend type constant for this type hint, if it maps to a
    /// simple constant.
    pub(crate) fn zend_type_const(&self) -> Option<u32> {
        match self {
            Self::Null => Some(IS_NULL),
            Self::Bool => Some(_IS_BOOL),
            Self::Int => Some(IS_LONG),
            Self::Float => Some(IS_DOUBLE),
            Self::String => Some(IS_STRING),
            Self::Array => Some(IS_ARRAY),
            Self::Object => Some(IS_OBJECT),
            Self::Callable => Some(IS_CALLABLE),
            Self::Iterable => Some(IS_ITERABLE),
            Self::Mixed => Some(IS_MIXED),
            Self::ClassEntry(_) | Self::Never | Self::Void | Self::False | Self::Union(_) => None,
        }
    }

    /// Returns the members of a checked union, or of a standalone false.
    pub(crate) fn union_members(&self) -> Result<Vec<UnionMember<'_>>, String> {
        match self {
            Self::Union(members) => {
                let members = members.iter().map(Self::union_member).collect::<Vec<_>>();
                check_union(&members)?;
                Ok(members)
            }
            hint => Ok(vec![hint.union_member()]),
        }
    }

    fn union_member(&self) -> UnionMember<'_> {
        match self {
            Self::Null => UnionMember::Null,
            Self::False => UnionMember::False,
            Self::Bool => UnionMember::Bool,
            Self::Int => UnionMember::Int,
            Self::Float => UnionMember::Float,
            Self::String => UnionMember::String,
            Self::Array => UnionMember::Array,
            Self::Object => UnionMember::Object,
            Self::Callable => UnionMember::Callable,
            Self::Iterable => UnionMember::Iterable,
            Self::ClassEntry(class_name) => UnionMember::Class(class_name),
            Self::Mixed => UnionMember::Standalone("mixed"),
            Self::Never => UnionMember::Standalone("never"),
            Self::Void => UnionMember::Standalone("void"),
            Self::Union(_) => UnionMember::Union,
        }
    }
}

/// One member of a union typehint, common to arguments and returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnionMember<'a> {
    Null,
    False,
    Bool,
    Int,
    Float,
    String,
    Array,
    Object,
    Callable,
    Iterable,
    Class(&'a str),
    /// mixed, void or never, which cannot be part of a union.
    Standalone(&'static str),
    Union,
}

impl Display for UnionMember<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Null => "null",
            Self::False => "false",
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::String => "string",
            Self::Array => "array",
            Self::Object => "object",
            Self::Callable => "callable",
            Self::Iterable => "iterable",
            Self::Class(class_name) => class_name,
            Self::Standalone(name) => name,
            Self::Union => "union",
        })
    }
}

/// Rejects the unions `zend_compile_typename` rejects in userland. Internal
/// registration checks none of them.
pub(crate) fn check_union(members: &[UnionMember<'_>]) -> Result<(), String> {
    use UnionMember::*;

    if members.len() < 2 {
        return Err("a union needs at least two members".to_owned());
    }
    for (i, member) in members.iter().enumerate() {
        match member {
            Standalone(name) => {
                return Err(format!("{} can only be used as a standalone type", name));
            }
            Union => return Err("a union cannot contain another union".to_owned()),
            _ => {}
        }
        for earlier in &members[..i] {
            if member.same_as(earlier) {
                return Err(format!("duplicate type {} is redundant", member));
            }
            if member.overlaps(earlier) || earlier.overlaps(member) {
                return Err(format!("{} is redundant with {}", member, earlier));
            }
        }
    }
    Ok(())
}

/// Lowers checked union members to the `|`-joined class names and type mask
/// that `ZEND_ARG_OBJ_TYPE_MASK` takes, the same way `build/gen_stub.php` does.
#[cfg(phper_major_version = "8")]
pub(crate) fn lower_union(members: &[UnionMember<'_>]) -> (Option<CString>, u32) {
    use UnionMember::*;

    let mut class_names = Vec::new();
    let mut type_mask = 0;
    for member in members {
        match member {
            Null => type_mask |= MAY_BE_NULL,
            False => type_mask |= MAY_BE_FALSE,
            Bool => type_mask |= MAY_BE_BOOL,
            Int => type_mask |= MAY_BE_LONG,
            Float => type_mask |= MAY_BE_DOUBLE,
            String => type_mask |= MAY_BE_STRING,
            Array => type_mask |= MAY_BE_ARRAY,
            Object => type_mask |= MAY_BE_OBJECT,
            Callable => type_mask |= MAY_BE_CALLABLE,
            #[cfg(any(phper_minor_version = "0", phper_minor_version = "1"))]
            Iterable => type_mask |= 1 << IS_ITERABLE,
            // From 8.2 zend_register_functions replaces any type carrying the
            // iterable bit with Traversable|array, dropping the other members.
            // Userland rewrites iterable the same way in
            // zend_compile_single_typename.
            #[cfg(not(any(phper_minor_version = "0", phper_minor_version = "1")))]
            Iterable => {
                class_names.push("Traversable");
                type_mask |= MAY_BE_ARRAY;
            }
            Class(class_name) => class_names.push(*class_name),
            Standalone(_) | Union => unreachable!("rejected by check_union"),
        }
    }
    let class_names = (!class_names.is_empty())
        .then(|| CString::new(class_names.join("|")).expect("CString::new failed"));
    (class_names, type_mask)
}

impl UnionMember<'_> {
    fn same_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Class(a), Self::Class(b)) => a.eq_ignore_ascii_case(b),
            (a, b) => a == b,
        }
    }

    /// Whether `other` is already allowed by `self`.
    fn overlaps(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool, Self::False) | (Self::Object, Self::Class(_)) => true,
            (Self::Iterable, Self::Array) => true,
            (Self::Iterable, Self::Class(class_name)) => {
                class_name.eq_ignore_ascii_case("Traversable")
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UnionMember::*, *};

    #[test]
    fn check_union_accepts_distinct_members() {
        assert_eq!(check_union(&[Class("Countable"), False, Null]), Ok(()));
        assert_eq!(check_union(&[Int, String]), Ok(()));
        assert_eq!(check_union(&[Iterable, Int]), Ok(()));
        assert_eq!(
            check_union(&[Class("Countable"), Class("Stringable")]),
            Ok(())
        );
    }

    #[test]
    fn check_union_rejects_fewer_than_two_members() {
        for members in [&[][..], &[Int][..]] {
            assert_eq!(
                check_union(members),
                Err("a union needs at least two members".to_owned())
            );
        }
    }

    #[test]
    fn union_members_accepts_standalone_false() {
        assert_eq!(ArgumentTypeHint::False.union_members(), Ok(vec![False]));
        assert_eq!(ReturnTypeHint::False.union_members(), Ok(vec![False]));
    }

    #[test]
    fn union_members_checks_unions() {
        assert_eq!(
            ArgumentTypeHint::Union(vec![ArgumentTypeHint::Int]).union_members(),
            Err("a union needs at least two members".to_owned())
        );
        assert_eq!(
            ReturnTypeHint::Union(vec![ReturnTypeHint::Int, ReturnTypeHint::Void]).union_members(),
            Err("void can only be used as a standalone type".to_owned())
        );
    }

    #[test]
    fn check_union_rejects_standalone_types() {
        for name in ["mixed", "void", "never"] {
            assert_eq!(
                check_union(&[Int, Standalone(name)]),
                Err(format!("{} can only be used as a standalone type", name))
            );
        }
    }

    #[test]
    fn check_union_rejects_nested_union() {
        assert_eq!(
            check_union(&[Int, Union]),
            Err("a union cannot contain another union".to_owned())
        );
    }

    #[test]
    fn check_union_rejects_duplicates() {
        assert_eq!(
            check_union(&[Int, String, Int]),
            Err("duplicate type int is redundant".to_owned())
        );
        assert_eq!(
            check_union(&[Class("Foo"), Int, Class("FOO")]),
            Err("duplicate type FOO is redundant".to_owned())
        );
    }

    #[test]
    fn check_union_rejects_overlapping_members() {
        let cases: &[(&[UnionMember<'_>], &str)] = &[
            (&[Bool, False], "false is redundant with bool"),
            (&[False, Bool], "bool is redundant with false"),
            (
                &[Object, Class("Countable")],
                "Countable is redundant with object",
            ),
            (
                &[Class("Countable"), Object],
                "object is redundant with Countable",
            ),
            (&[Iterable, Array], "array is redundant with iterable"),
            (
                &[Iterable, Class("Traversable")],
                "Traversable is redundant with iterable",
            ),
            (
                &[Class("traversable"), Iterable],
                "iterable is redundant with traversable",
            ),
        ];
        for (members, message) in cases {
            assert_eq!(check_union(members), Err((*message).to_owned()));
        }
    }
}
