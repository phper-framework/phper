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
    errors::{ThrowObject, exception_class},
    modules::Module,
};
use std::io;

pub fn integrate(module: &mut Module) {
    {
        let e = phper::Error::boxed("something wrong");
        assert!(matches!(e, phper::Error::Boxed(..)));
        assert_eq!(e.to_string(), "something wrong");
    }

    {
        let e = phper::Error::boxed(io::Error::new(io::ErrorKind::Other, "oh no!"));
        assert!(matches!(e, phper::Error::Boxed(..)));
        assert_eq!(e.to_string(), "oh no!");
    }

    module.add_function("integrate_throw_boxed", |_arguments| {
        Err::<(), _>(phper::Error::boxed("What's wrong with you?"))
    });

    module.add_function("integrate_throw_object", |_arguments| {
        let obj = exception_class().new_object(["Forbidden".into(), 403.into()])?;
        let obj = ThrowObject::new(obj)?;
        Err::<(), _>(phper::Error::Throw(obj))
    });
}
