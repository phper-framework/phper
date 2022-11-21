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
        argument::{Argument, ArgumentSet},
        data_type::{AtomicReturnType, AtomicType},
        describe::Describe,
        handler::{Context, FunctionHandler, State},
        value::Value,
    },
    errors::Result,
};

pub struct Function<F>
where
    F: Fn(State, Context, ArgumentSet) -> Result<Value>,
{
    pub name: String,
    pub arguments: Vec<Argument>,
    pub returns: AtomicReturnType,
    pub handler: FunctionHandler<F>,
}

impl<F> Function<F>
where
    F: Fn(State, Context, ArgumentSet) -> Result<Value>,
{
    pub fn new(name: String, handler: FunctionHandler<F>) -> Function<F> {
        Function {
            name,
            arguments: vec![],
            returns: AtomicReturnType::Of(AtomicType::Mixed),
            handler,
        }
    }

    pub fn with_argument(mut self, argument: Argument) -> Self {
        self.arguments.push(argument);

        self
    }

    pub fn returns(mut self, return_type: AtomicReturnType) -> Self {
        self.returns = return_type;

        self
    }
}

/// Describe `Function`.
///
/// Example:
///
/// ```rust, no_run
/// use phper::builder::{
///     argument::Argument,
///     data_type::{AtomicReturnType, AtomicType},
///     describe::Describe,
///     function::Function,
///     handler::{Context, FunctionHandler, State},
///     value::{ConstantValue, Value},
/// };
///
/// let handler = FunctionHandler(|_, _, arguments| {
///     let mut message: String = arguments.get("message")?.value()?.try_into()?;
///     let args: Vec<String> = arguments.get("args")?.value()?.try_into()?;
///     for (i, arg) in args.iter().enumerate() {
///         // replace `{n}` with the args[n]
///         message = message.replace(&format!("[{i}]"), arg);
///     }
///
///     Ok(ConstantValue::String(message).into())
/// });
///
/// let function = Function::new("format".to_owned(), handler)
///     .with_argument(Argument::new("message", AtomicType::String))
///     .with_argument(Argument::varadic("args", AtomicType::String))
///     .returns(AtomicReturnType::Of(AtomicType::String));
///
/// assert_eq!(
///     "function format(string $message, string ...$args): string { exit(1); }",
///     function.describe()
/// );
/// ```
impl<F> Describe for Function<F>
where
    F: Fn(State, Context, ArgumentSet) -> Result<Value>,
{
    fn describe(&self) -> String {
        format!(
            "function {}({}): {} {{ exit(1); }}",
            self.name,
            self.arguments
                .iter()
                .map(|a| a.describe())
                .collect::<Vec<String>>()
                .join(", "),
            self.returns.describe()
        )
    }
}
