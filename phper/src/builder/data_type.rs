// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::builder::{argument::Argument, describe::Describe};

pub enum ArrayKeyType {
    String,
    Int,
    Mixed,
}

pub enum AtomicType {
    Bool,
    Int,
    Float,
    String,
    NonEmptyString,
    IntRange(i64, i64),
    ClassString,
    ClassStringOf(String),
    TraitString,
    EnumString,
    NumericString,
    LiteralString,
    LiteralInt,
    Numeric,
    Scalar,
    Object,
    ObjectOf(String),
    List(Box<AtomicType>),
    Array(Box<(ArrayKeyType, AtomicType)>),
    NonEmptyList(Box<AtomicType>),
    NonEmptyArray(Box<(ArrayKeyType, AtomicType)>),
    Shape(Vec<(ArrayKeyType, AtomicType)>),
    Iterable(Box<(ArrayKeyType, AtomicType)>),
    Null,
    True,
    False,
    Resource,
    ClosedResource,
    Mixed,
    Union(Vec<AtomicType>),
    Function(Vec<Argument>, Box<AtomicReturnType>),
}

impl AtomicType {
    pub fn list(value_type: AtomicType) -> AtomicType {
        AtomicType::List(Box::new(value_type))
    }

    pub fn non_empty_list(value_type: AtomicType) -> AtomicType {
        AtomicType::List(Box::new(value_type))
    }

    pub fn array(key_type: ArrayKeyType, value_type: AtomicType) -> AtomicType {
        AtomicType::Array(Box::new((key_type, value_type)))
    }

    pub fn non_empty_array(key_type: ArrayKeyType, value_type: AtomicType) -> AtomicType {
        AtomicType::Array(Box::new((key_type, value_type)))
    }

    pub fn iterable(key_type: ArrayKeyType, value_type: AtomicType) -> AtomicType {
        AtomicType::Iterable(Box::new((key_type, value_type)))
    }

    pub fn function(arguments: Vec<Argument>, return_type: AtomicReturnType) -> AtomicType {
        AtomicType::Function(arguments, Box::new(return_type))
    }
}

pub enum AtomicReturnType {
    Of(AtomicType),
    Void,
    Never,
}

impl Describe for AtomicType {
    fn describe(&self) -> String {
        match self {
            AtomicType::Bool => "bool".to_string(),
            AtomicType::Int => "int".to_string(),
            AtomicType::Float => "float".to_string(),
            AtomicType::String => "string".to_string(),
            AtomicType::NonEmptyString => "string".to_string(),
            AtomicType::IntRange(_, _) => "int".to_string(),
            AtomicType::ClassString => "string".to_string(),
            AtomicType::ClassStringOf(_) => "string".to_string(),
            AtomicType::TraitString => "string".to_string(),
            AtomicType::EnumString => "string".to_string(),
            AtomicType::NumericString => "string".to_string(),
            AtomicType::LiteralString => "string".to_string(),
            AtomicType::LiteralInt => "int".to_string(),
            AtomicType::Numeric => "int|float|string".to_string(),
            AtomicType::Scalar => "int|float|string|bool".to_string(),
            AtomicType::Object => "object".to_string(),
            AtomicType::ObjectOf(classname) => classname.to_string(),
            AtomicType::List(_) => "array".to_string(),
            AtomicType::Array(_) => "array".to_string(),
            AtomicType::NonEmptyList(_) => "array".to_string(),
            AtomicType::NonEmptyArray(_) => "array".to_string(),
            AtomicType::Shape(_) => "array".to_string(),
            AtomicType::Iterable(_) => "iterable".to_string(),
            AtomicType::Null => {
                if cfg!(all(phper_major_version = "8", phper_minor_version = "2")) {
                    "null".to_string()
                } else {
                    "mixed".to_string()
                }
            }
            AtomicType::True => {
                if cfg!(all(phper_major_version = "8", phper_minor_version = "2")) {
                    "true".to_string()
                } else {
                    "bool".to_string()
                }
            }
            AtomicType::False => {
                if cfg!(all(phper_major_version = "8", phper_minor_version = "2")) {
                    "false".to_string()
                } else {
                    "bool".to_string()
                }
            }
            AtomicType::Resource => "mixed".to_string(),
            AtomicType::ClosedResource => "mixed".to_string(),
            AtomicType::Mixed => "mixed".to_string(),
            AtomicType::Union(types) => {
                let types = types
                    .iter()
                    .map(|atomic_type| atomic_type.describe())
                    .collect::<Vec<String>>();

                types.join("|")
            }
            AtomicType::Function(_, _) => "Closure".to_string(),
        }
    }
}

/// Describe `AtomicReturnType`.
///
/// Example:
///
/// ```rust, no_run
/// use phper::builder::{
///     data_type::{AtomicReturnType, AtomicType},
///     describe::Describe,
/// };
///
/// let t = AtomicReturnType::Of(AtomicType::Mixed);
///
/// assert_eq!("mixed", t.describe());
///
/// let t = AtomicReturnType::Void;
///
/// assert_eq!("void", t.describe());
///
/// let t = AtomicReturnType::Never;
///
/// assert_eq!("never", t.describe());
/// ```
impl Describe for AtomicReturnType {
    fn describe(&self) -> String {
        match self {
            AtomicReturnType::Of(atomic_type) => atomic_type.describe(),
            AtomicReturnType::Void => "void".to_string(),
            AtomicReturnType::Never => "never".to_string(),
        }
    }
}
