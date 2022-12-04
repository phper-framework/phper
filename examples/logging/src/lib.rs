// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use anyhow::Context;
use phper::{
    deprecated, echo, error, functions::Argument, modules::Module, notice, php_get_module,
    values::ZVal, warning,
};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module
        .add_function("log_say", |params: &mut [ZVal]| -> phper::Result<()> {
            params[0].convert_to_string();
            let message = params[0].as_z_str().unwrap().to_str().context("to str")?;
            echo!("Hello, {}!", message);
            Ok(())
        })
        .argument(Argument::by_val("message"));

    module
        .add_function("log_notice", |params: &mut [ZVal]| -> phper::Result<()> {
            params[0].convert_to_string();
            let message = params[0].as_z_str().unwrap().to_str().context("to str")?;
            notice!("Something happened: {}", message);
            Ok(())
        })
        .argument(Argument::by_val("message"));

    module
        .add_function("log_warning", |params: &mut [ZVal]| -> phper::Result<()> {
            params[0].convert_to_string();
            let message = params[0].as_z_str().unwrap().to_str().context("to str")?;
            warning!("Something warning: {}", message);
            Ok(())
        })
        .argument(Argument::by_val("message"));

    module
        .add_function("log_error", |params: &mut [ZVal]| -> phper::Result<()> {
            params[0].convert_to_string();
            let message = params[0].as_z_str().unwrap().to_str().context("to str")?;
            error!("Something gone failed: {}", message);
            Ok(())
        })
        .argument(Argument::by_val("message"));

    module
        .add_function(
            "log_deprecated",
            |params: &mut [ZVal]| -> phper::Result<()> {
                params[0].convert_to_string();
                let message = params[0].as_z_str().unwrap().to_str().context("to str")?;
                deprecated!("Something deprecated: {}", message);
                Ok(())
            },
        )
        .argument(Argument::by_val("message"));

    module
}
