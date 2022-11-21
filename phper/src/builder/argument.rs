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
    builder::{
        data_type::AtomicType,
        describe::Describe,
        value::{ConstantValue, Value},
    },
    errors::{Error, Result},
};

pub enum ArgumentFlag {
    IsVaradic,
    HasDefault(ConstantValue),
}

pub struct Argument {
    pub name: String,
    pub data_type: AtomicType,
    pub flag: Option<ArgumentFlag>,
    pub value: Option<Value>,
}

impl Argument {
    pub fn new(name: &str, data_type: AtomicType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            flag: None,
            value: None,
        }
    }

    pub fn varadic(name: &str, data_type: AtomicType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            flag: Some(ArgumentFlag::IsVaradic),
            value: None,
        }
    }

    pub fn with_default(mut self, value: ConstantValue) -> Self {
        self.flag = Some(ArgumentFlag::HasDefault(value));

        self
    }

    pub fn value(&self) -> Result<Value> {
        if let Some(v) = &self.value {
            Ok(v.clone())
        } else if let Some(ArgumentFlag::HasDefault(default)) = &self.flag {
            let value = default.clone().into();

            Ok(value)
        } else {
            Err(Error::other(
                "attempted to access argument value prior to initlization.",
            ))
        }
    }
}

/// Describe `Argument`.
///
/// Example:
///
/// ```rust, no_run
/// use phper::builder::{
///     argument::{Argument, ArgumentFlag},
///     data_type::AtomicType,
///     describe::Describe,
///     value::ConstantValue,
/// };
///
/// let argument = Argument::new("foo", AtomicType::Scalar);
///
/// assert_eq!("int|float|string|bool $foo", argument.describe());
///
/// let argument = Argument::new("foo", AtomicType::list(AtomicType::Scalar)).with_default(
///     ConstantValue::List(vec![
///         ("hello, world!").into(),
///         (15).into(),
///         (124.5).into(),
///         (false).into(),
///         (true).into(),
///     ]),
/// );
///
/// assert_eq!(
///     "array $foo = ['hello, world!', 15, 124.5, false, true]",
///     argument.describe()
/// );
///
/// let argument = Argument::varadic("foo", AtomicType::Scalar);
///
/// assert_eq!("int|float|string|bool ...$foo", argument.describe());
/// ```
impl Describe for Argument {
    fn describe(&self) -> String {
        match &self.flag {
            Some(ArgumentFlag::IsVaradic) => {
                format!("{} ...${}", self.data_type.describe(), self.name)
            }
            Some(ArgumentFlag::HasDefault(value)) => {
                format!(
                    "{} ${} = {}",
                    self.data_type.describe(),
                    self.name,
                    value.describe()
                )
            }
            None => {
                format!("{} ${}", self.data_type.describe(), self.name)
            }
        }
    }
}

pub struct ArgumentSet {
    pub arguments: Vec<Argument>,
}

impl ArgumentSet {
    pub fn nth(&self, index: usize) -> Result<&Argument> {
        self.arguments
            .get(index)
            .ok_or_else(|| Error::other("undefined argument"))
    }

    pub fn get(&self, name: &str) -> Result<&Argument> {
        for argument in &self.arguments {
            if argument.name == name {
                return Ok(argument);
            }
        }

        Err(Error::other("undefined argument"))
    }
}
