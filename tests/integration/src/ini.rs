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
    ini::{Policy, ini_get},
    modules::Module,
};
use std::{convert::Infallible, ffi::CStr};

pub fn integrate(module: &mut Module) {
    module.add_ini("INTEGRATE_INI_TRUE", true, Policy::System);
    module.add_ini("INTEGRATE_INI_FALSE", false, Policy::System);
    module.add_ini("INTEGRATE_INI_LONG", 100i64, Policy::System);
    module.add_ini("INTEGRATE_INI_DOUBLE", 200., Policy::System);
    module.add_ini(
        "INTEGRATE_INI_STRING",
        "something".to_owned(),
        Policy::System,
    );

    module.add_function("integrate_ini_assert", |_| {
        assert!(ini_get::<bool>("INTEGRATE_INI_TRUE"));
        assert!(!ini_get::<bool>("INTEGRATE_INI_FALSE"));
        assert_eq!(ini_get::<i64>("INTEGRATE_INI_LONG"), 100);
        assert_eq!(ini_get::<f64>("INTEGRATE_INI_DOUBLE"), 200.);
        assert_eq!(
            ini_get::<Option<&CStr>>("INTEGRATE_INI_STRING"),
            Some(c"something")
        );
        Ok::<_, Infallible>(())
    });
}
