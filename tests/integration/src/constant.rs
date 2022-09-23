// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::modules::Module;

pub fn integrate(module: &mut Module) {
    module.add_constant("INTEGRATE_CONST_NULL", ());
    module.add_constant("INTEGRATE_CONST_TRUE", true);
    module.add_constant("INTEGRATE_CONST_FALSE", false);
    module.add_constant("INTEGRATE_CONST_LONG", 100i64);
    module.add_constant("INTEGRATE_CONST_DOUBLE", 200.);
    module.add_constant("INTEGRATE_CONST_STRING", "something");
    module.add_constant("INTEGRATE_CONST_BYTES", "something".as_bytes().to_owned());
}
