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
    alloc::{ebox, EBox},
    arrays::{InsertKey, ZArray},
    modules::Module,
    objects::ZObject,
    values::ZVal,
};

pub fn integrate(module: &mut Module) {
    integrate_returns(module);
    integrate_as(module);
}

fn integrate_returns(module: &mut Module) {
    module.add_function(
        "integration_values_return_null",
        integration_values_return_null,
    );
    module.add_function(
        "integration_values_return_true",
        integration_values_return_true,
    );
    module.add_function(
        "integration_values_return_false",
        integration_values_return_false,
    );
    module.add_function(
        "integration_values_return_i64",
        integration_values_return_i64,
    );
    module.add_function(
        "integration_values_return_f64",
        integration_values_return_f64,
    );
    module.add_function(
        "integration_values_return_str",
        integration_values_return_str,
    );
    module.add_function(
        "integration_values_return_string",
        integration_values_return_string,
    );
    module.add_function(
        "integration_values_return_array",
        integration_values_return_array,
    );
    module.add_function(
        "integration_values_return_kv_array",
        integration_values_return_kv_array,
    );
    module.add_function(
        "integration_values_return_object",
        integration_values_return_object,
    );
    module.add_function(
        "integration_values_return_option_i64_some",
        integration_values_return_option_i64_some,
    );
    module.add_function(
        "integration_values_return_option_i64_none",
        integration_values_return_option_i64_none,
    );
    module.add_function(
        "integration_values_return_ebox_i64",
        integration_values_return_ebox_i64,
    );
    module.add_function(
        "integration_values_return_result_string_ok",
        integration_values_return_result_string_ok,
    );
    module.add_function(
        "integration_values_return_result_string_err",
        integration_values_return_result_string_err,
    );
    module.add_function(
        "integration_values_return_val",
        integration_values_return_val,
    );
}

fn integration_values_return_null(_: &mut [ZVal]) {}

fn integration_values_return_true(_: &mut [ZVal]) -> bool {
    true
}

fn integration_values_return_false(_: &mut [ZVal]) -> bool {
    false
}

fn integration_values_return_i64(_: &mut [ZVal]) -> i64 {
    64
}

fn integration_values_return_f64(_: &mut [ZVal]) -> f64 {
    64.0
}

fn integration_values_return_str(_: &mut [ZVal]) -> &'static str {
    "foo"
}

fn integration_values_return_string(_: &mut [ZVal]) -> String {
    "foo".to_string()
}

fn integration_values_return_array(_: &mut [ZVal]) -> ZArray {
    let mut arr = ZArray::new();
    arr.insert(InsertKey::NextIndex, ZVal::from("a"));
    arr.insert(InsertKey::NextIndex, ZVal::from("b"));
    arr.insert(InsertKey::NextIndex, ZVal::from("c"));
    arr
}

fn integration_values_return_kv_array(_: &mut [ZVal]) -> ZArray {
    let mut arr = ZArray::new();
    arr.insert("a", ZVal::from(1));
    arr.insert("b", ZVal::from("foo"));
    arr
}

fn integration_values_return_object(_: &mut [ZVal]) -> ZObject {
    let mut object = ZObject::new_by_std_class();
    object.set_property("foo", ZVal::from("bar"));
    object
}

fn integration_values_return_option_i64_some(_: &mut [ZVal]) -> Option<i64> {
    Some(64)
}

fn integration_values_return_option_i64_none(_: &mut [ZVal]) -> Option<i64> {
    None
}

fn integration_values_return_ebox_i64(_: &mut [ZVal]) -> EBox<i64> {
    ebox!(64)
}

fn integration_values_return_result_string_ok(_: &mut [ZVal]) -> phper::Result<String> {
    Ok("foo".to_string())
}

fn integration_values_return_result_string_err(_: &mut [ZVal]) -> phper::Result<()> {
    Err(phper::Error::Boxed("a zhe".into()))
}

fn integration_values_return_val(_: &mut [ZVal]) -> ZVal {
    ZVal::from("foo")
}

fn integrate_as(_module: &mut Module) {
    {
        let val = ZVal::default();
        assert_eq!(val.as_null(), Some(()));
        assert_eq!(val.as_long(), None);
    }

    {
        let val = ZVal::from(true);
        assert_eq!(val.as_bool(), Some(true));
        assert_eq!(val.as_long(), None);
    }

    {
        let mut val = ZVal::from(100i64);
        assert_eq!(val.as_long(), Some(100));
        assert_eq!(val.as_double(), None);
        if let Some(l) = val.as_mut_long() {
            *l += 100;
        }
        assert_eq!(val.as_long(), Some(200));
    }

    {
        let mut val = ZVal::from(100f64);
        assert_eq!(val.as_double(), Some(100.));
        assert_eq!(val.as_long(), None);
        if let Some(d) = val.as_mut_double() {
            *d += 100.;
        }
        assert_eq!(val.as_double(), Some(200.));
    }
}
