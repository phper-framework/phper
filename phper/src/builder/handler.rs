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
    builder::{argument::ArgumentSet, value::Value},
    errors::Result,
};

/// TODO
pub struct Context {}
/// TODO
pub struct State {}
/// TODO
pub struct Instance {}

pub struct FunctionHandler<F>(pub F)
where
    F: Fn(State, Context, ArgumentSet) -> Result<Value>;

pub struct MethodFunctionHandler<F>(pub F)
where
    F: Fn(Instance, State, Context, ArgumentSet) -> Result<Value>;
