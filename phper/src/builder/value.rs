// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{
    arrays::{InsertKey, ZArray},
    builder::describe::Describe,
    values::ZVal,
    Error,
};

#[derive(Clone)]
pub struct Value {
    zval: ZVal,
}

impl Value {
    pub fn from_zval(zval: ZVal) -> Self {
        Self { zval }
    }
}

impl From<ConstantValue> for Value {
    fn from(constant: ConstantValue) -> Self {
        Value {
            zval: constant.into(),
        }
    }
}

macro_rules! implement_try_into {
    ($type:ty, $fn:ident) => {
        impl TryInto<$type> for Value {
            type Error = Error;

            fn try_into(self) -> Result<$type, Self::Error> {
                self.zval.$fn()
            }
        }
    };
}

implement_try_into!(i64, expect_long);
implement_try_into!(f64, expect_double);
implement_try_into!(bool, expect_bool);
implement_try_into!((), expect_null);

impl TryInto<String> for Value {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(self.zval.expect_z_str()?.to_str()?.to_string())
    }
}

impl TryInto<Vec<String>> for Value {
    type Error = Error;

    fn try_into(mut self) -> Result<Vec<String>, Self::Error> {
        self.zval
            .expect_mut_z_arr()?
            .iter()
            .map(|v| Ok::<String, Error>(v.1.expect_z_str()?.to_str()?.to_string()))
            .collect::<Result<Vec<String>, Error>>()
    }
}

#[derive(Clone)]
pub enum ArrayKeyValue {
    String(String),
    Int(u64),
}

#[derive(Clone)]
pub enum ConstantValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<ConstantValue>),
    Array(Vec<(ArrayKeyValue, ConstantValue)>),
    Constant(String),
    ClassConstant(String, String),
    EnumCase(String, String),
    Null,
}

impl From<ConstantValue> for ZVal {
    fn from(constant: ConstantValue) -> Self {
        match constant {
            ConstantValue::String(value) => value.into(),
            ConstantValue::Int(value) => value.into(),
            ConstantValue::Float(value) => value.into(),
            ConstantValue::Bool(value) => value.into(),
            ConstantValue::List(values) => {
                let mut array = ZArray::new();
                for value in values {
                    array.insert(InsertKey::NextIndex, value.into());
                }

                array.into()
            }
            ConstantValue::Array(values) => {
                let mut array = ZArray::new();
                for value in values {
                    match value.0 {
                        ArrayKeyValue::String(k) => {
                            array.insert(InsertKey::Str(&k), value.1.into());
                        }
                        ArrayKeyValue::Int(k) => {
                            array.insert(InsertKey::Index(k), value.1.into());
                        }
                    }
                }

                array.into()
            }
            ConstantValue::Constant(_) => todo!("not sure how yet."),
            ConstantValue::ClassConstant(_, _) => todo!("not sure how yet."),
            ConstantValue::EnumCase(_, _) => todo!("not sure how yet."),
            ConstantValue::Null => ().into(),
        }
    }
}

/// Describe `ArrayKeyValue`
///
/// ```rust, no_run
/// use phper::builder::{describe::Describe, value::ArrayKeyValue};
///
/// let k = ArrayKeyValue::String("'foo".to_string());
///
/// assert_eq!("'\\'foo'", k.describe());
///
/// let k = ArrayKeyValue::Int(12);
///
/// assert_eq!("12", k.describe());
/// ```
impl Describe for ArrayKeyValue {
    fn describe(&self) -> String {
        match self {
            ArrayKeyValue::String(value) => format!("'{}'", value.replace('\'', "\\'")),
            ArrayKeyValue::Int(value) => format!("{}", value),
        }
    }
}

/// Describe `ConstantValue`
///
/// ```rust, no_run
/// use phper::builder::{
///     describe::Describe,
///     value::{ArrayKeyValue, ConstantValue},
/// };
///
/// let v = ConstantValue::String("'foo".to_string());
///
/// assert_eq!("'\\'foo'", v.describe());
///
/// let v = ConstantValue::Int(12);
///
/// assert_eq!("12", v.describe());
///
/// let v = ConstantValue::Float(12.123);
///
/// assert_eq!("12.123", v.describe());
///
/// let v = ConstantValue::Bool(false);
///
/// assert_eq!("false", v.describe());
///
/// let v = ConstantValue::Bool(true);
///
/// assert_eq!("true", v.describe());
///
/// let v = ConstantValue::Null;
///
/// assert_eq!("null", v.describe());
///
/// let v = ConstantValue::List(vec![
///     ConstantValue::String("foo".to_string()),
///     ConstantValue::String("bar".to_string()),
///     ConstantValue::String("baz".to_string()),
///     ConstantValue::List(vec![
///         ConstantValue::Int(1),
///         ConstantValue::Int(2),
///         ConstantValue::Int(3),
///         ().into(),
///         ("Foo", "Bar").into(),
///     ]),
/// ]);
///
/// assert_eq!(
///     "['foo', 'bar', 'baz', [1, 2, 3, null, Foo::Bar]]",
///     v.describe()
/// );
///
/// let v = ConstantValue::Array(vec![
///     (("foo").into(), (1).into()),
///     (("bar").into(), (2).into()),
///     (("baz").into(), (3).into()),
/// ]);
///
/// assert_eq!("['foo' => 1, 'bar' => 2, 'baz' => 3]", v.describe());
///
/// let v = ConstantValue::Constant("Psl\\Str\\ALPHABET".to_string());
///
/// assert_eq!("Psl\\Str\\ALPHABET", v.describe());
///
/// let v = ConstantValue::ClassConstant("FooClass".to_string(), "BAR_CONSTANT".to_string());
///
/// assert_eq!("FooClass::BAR_CONSTANT", v.describe());
///
/// let v = ConstantValue::EnumCase("Psl\\Str\\Encoding".to_string(), "UTF_8".to_string());
///
/// assert_eq!("Psl\\Str\\Encoding::UTF_8", v.describe());
/// ```
impl Describe for ConstantValue {
    fn describe(&self) -> String {
        match self {
            ConstantValue::String(value) => format!("'{}'", value.replace('\'', "\\'")),
            ConstantValue::Int(value) => format!("{}", value),
            ConstantValue::Float(value) => format!("{}", value),
            ConstantValue::Bool(value) => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            ConstantValue::List(value) => {
                format!(
                    "[{}]",
                    value
                        .iter()
                        .map(|v| v.describe())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ConstantValue::Array(value) => {
                format!(
                    "[{}]",
                    value
                        .iter()
                        .map(|v| format!("{} => {}", v.0.describe(), v.1.describe()))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ConstantValue::Null => "null".to_string(),
            ConstantValue::Constant(constant) => constant.to_string(),
            ConstantValue::ClassConstant(class, constant) => format!("{}::{}", class, constant),
            ConstantValue::EnumCase(class, case) => format!("{}::{}", class, case),
        }
    }
}

impl From<()> for ConstantValue {
    fn from(_: ()) -> Self {
        ConstantValue::Null
    }
}

impl From<(String, String)> for ConstantValue {
    fn from(value: (String, String)) -> Self {
        ConstantValue::ClassConstant(value.0, value.1)
    }
}
impl From<(&'static str, &'static str)> for ConstantValue {
    fn from(value: (&'static str, &'static str)) -> Self {
        ConstantValue::ClassConstant(value.0.to_string(), value.1.to_string())
    }
}

macro_rules! implement_from {
    ($type:ty, $to:ident, $field:ident) => {
        impl From<$type> for $to {
            fn from(value: $type) -> Self {
                $to::$field(value.into())
            }
        }
    };
}

implement_from!(String, ArrayKeyValue, String);
implement_from!(&'static str, ArrayKeyValue, String);
implement_from!(u16, ArrayKeyValue, Int);
implement_from!(u32, ArrayKeyValue, Int);
implement_from!(u64, ArrayKeyValue, Int);

implement_from!(String, ConstantValue, String);
implement_from!(&'static str, ConstantValue, String);
implement_from!(i16, ConstantValue, Int);
implement_from!(i32, ConstantValue, Int);
implement_from!(i64, ConstantValue, Int);
implement_from!(f32, ConstantValue, Float);
implement_from!(f64, ConstantValue, Float);
implement_from!(bool, ConstantValue, Bool);
implement_from!(Vec<ConstantValue>, ConstantValue, List);
implement_from!(Vec<(ArrayKeyValue, ConstantValue)>, ConstantValue, Array);
