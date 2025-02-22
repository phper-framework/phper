// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{
    arrays::ZArray,
    errors::throw,
    functions::{Argument, call},
    modules::Module,
    values::ZVal,
};
use std::{convert::Infallible, io};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_functions_call",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut arr = ZArray::new();
            arr.insert("a", ZVal::from(1));
            arr.insert("b", ZVal::from(2));
            let ret = call("array_sum", &mut [ZVal::from(arr)])?;
            assert_eq!(ret.expect_long()?, 3);
            Ok(())
        },
    );

    module
        .add_function(
            "integrate_functions_call_callable",
            |arguments: &mut [ZVal]| {
                if let [head, tail @ ..] = arguments {
                    Ok::<_, phper::Error>(head.call(tail)?)
                } else {
                    unreachable!()
                }
            },
        )
        .argument(Argument::by_val("fn"));

    module.add_function(
        "integrate_functions_throw_error_exception",
        |_| -> phper::Result<()> { Err(phper::Error::boxed("throw error exception")) },
    );

    module.add_function("integrate_functions_exception_guard", |_| {
        unsafe {
            throw(phper::Error::Io(io::Error::new(
                io::ErrorKind::Other,
                "other io error",
            )));
        }
        let e = call("integrate_functions_throw_error_exception", []).unwrap_err();
        assert_eq!(e.to_string(), "throw error exception");
        Ok::<_, Infallible>(())
    });
}
