// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{functions::Argument, modules::Module, values::ZVal};

pub fn integrate(module: &mut Module) {
    materialize_missing_fill(module);
    materialize_missing_noop(module);
    materialize_missing_partial(module);
    materialize_missing_exceed_error(module);
    materialize_missing_insufficient_error(module);
}

fn materialize_missing_fill(module: &mut Module) {
    module
        .add_function_with_execute_data(
            "materialize_missing_fill",
            |execute_data, _arguments| -> phper::Result<String> {
                execute_data.materialize_missing([ZVal::from(42), ZVal::from("hello")])?;
                let a = execute_data.get_parameter(0).expect_long()?;
                let b = execute_data.get_parameter(1).expect_z_str()?.to_str()?;
                Ok(format!("{}, {}", a, b))
            },
        )
        .arguments([Argument::new("a").optional(), Argument::new("b").optional()]);
}

fn materialize_missing_noop(module: &mut Module) {
    module
        .add_function_with_execute_data(
            "materialize_missing_noop",
            |execute_data, _arguments| -> phper::Result<String> {
                let passed = execute_data.num_args();
                execute_data.materialize_missing([])?;
                let a = execute_data.get_parameter(0).expect_long()?;
                let b = execute_data.get_parameter(1).expect_z_str()?.to_str()?;
                Ok(format!("{}, {}, {}", passed, a, b))
            },
        )
        .arguments([Argument::new("a"), Argument::new("b")]);
}

fn materialize_missing_partial(module: &mut Module) {
    module
        .add_function_with_execute_data(
            "materialize_missing_partial",
            |execute_data, _arguments| -> phper::Result<String> {
                execute_data.materialize_missing([ZVal::from(42)])?;
                let a = execute_data.get_parameter(0).expect_z_str()?.to_str()?;
                let b = execute_data.get_parameter(1).expect_long()?;
                Ok(format!("{}, {}", a, b))
            },
        )
        .arguments([Argument::new("a").optional(), Argument::new("b").optional()]);
}

fn materialize_missing_exceed_error(module: &mut Module) {
    module
        .add_function_with_execute_data(
            "materialize_missing_exceed_error",
            |execute_data, _arguments| -> phper::Result<String> {
                execute_data.materialize_missing([ZVal::from(1), ZVal::from(2), ZVal::from(3)])?;
                Ok("ok".to_owned())
            },
        )
        .arguments([Argument::new("a").optional(), Argument::new("b").optional()]);
}

fn materialize_missing_insufficient_error(module: &mut Module) {
    module
        .add_function_with_execute_data(
            "materialize_missing_insufficient_error",
            |execute_data, _arguments| -> phper::Result<String> {
                execute_data.materialize_missing(std::iter::empty())?;
                Ok("ok".to_owned())
            },
        )
        .arguments([Argument::new("a").optional(), Argument::new("b").optional()]);
}
