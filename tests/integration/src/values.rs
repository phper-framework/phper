// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use indexmap::map::IndexMap;
use phper::{alloc::EBox, arrays::ZArray, modules::Module, objects::Object, values::ZVal};
use std::collections::HashMap;

pub fn integrate(module: &mut Module) {
    integrate_returns(module);
}

fn integrate_returns(module: &mut Module) {
    module.add_function(
        "integration_values_return_null",
        integration_values_return_null,
        vec![],
    );
    module.add_function(
        "integration_values_return_true",
        integration_values_return_true,
        vec![],
    );
    module.add_function(
        "integration_values_return_false",
        integration_values_return_false,
        vec![],
    );
    module.add_function(
        "integration_values_return_i32",
        integration_values_return_i32,
        vec![],
    );
    module.add_function(
        "integration_values_return_u32",
        integration_values_return_u32,
        vec![],
    );
    module.add_function(
        "integration_values_return_i64",
        integration_values_return_i64,
        vec![],
    );
    module.add_function(
        "integration_values_return_f64",
        integration_values_return_f64,
        vec![],
    );
    module.add_function(
        "integration_values_return_str",
        integration_values_return_str,
        vec![],
    );
    module.add_function(
        "integration_values_return_string",
        integration_values_return_string,
        vec![],
    );
    module.add_function(
        "integration_values_return_i64_vec",
        integration_values_return_i64_vec,
        vec![],
    );
    module.add_function(
        "integration_values_return_string_vec",
        integration_values_return_string_vec,
        vec![],
    );
    module.add_function(
        "integration_values_return_i64_map",
        integration_values_return_i64_map,
        vec![],
    );
    module.add_function(
        "integration_values_return_string_map",
        integration_values_return_string_map,
        vec![],
    );
    module.add_function(
        "integration_values_return_i64_index_map",
        integration_values_return_i64_index_map,
        vec![],
    );
    module.add_function(
        "integration_values_return_array",
        integration_values_return_array,
        vec![],
    );
    module.add_function(
        "integration_values_return_object",
        integration_values_return_object,
        vec![],
    );
    module.add_function(
        "integration_values_return_option_i64_some",
        integration_values_return_option_i64_some,
        vec![],
    );
    module.add_function(
        "integration_values_return_option_i64_none",
        integration_values_return_option_i64_none,
        vec![],
    );
    module.add_function(
        "integration_values_return_result_string_ok",
        integration_values_return_result_string_ok,
        vec![],
    );
    module.add_function(
        "integration_values_return_result_string_err",
        integration_values_return_result_string_err,
        vec![],
    );
    module.add_function(
        "integration_values_return_val",
        integration_values_return_val,
        vec![],
    );
}

fn integration_values_return_null(_: &mut [ZVal]) {}

fn integration_values_return_true(_: &mut [ZVal]) -> bool {
    true
}

fn integration_values_return_false(_: &mut [ZVal]) -> bool {
    false
}

fn integration_values_return_i32(_: &mut [ZVal]) -> i32 {
    32
}

fn integration_values_return_u32(_: &mut [ZVal]) -> u32 {
    32
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

fn integration_values_return_i64_vec(_: &mut [ZVal]) -> Vec<i64> {
    vec![0, 1, 2]
}

fn integration_values_return_string_vec(_: &mut [ZVal]) -> Vec<String> {
    vec!["a".to_string(), "b".to_string(), "c".to_string()]
}

fn integration_values_return_i64_map(_: &mut [ZVal]) -> HashMap<&'static str, i64> {
    let mut map = HashMap::new();
    map.insert("a", 0);
    map.insert("b", 1);
    map.insert("c", 2);
    map
}

fn integration_values_return_string_map(_: &mut [ZVal]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("a".to_string(), "x".to_string());
    map.insert("b".to_string(), "y".to_string());
    map.insert("c".to_string(), "z".to_string());
    map
}

fn integration_values_return_i64_index_map(_: &mut [ZVal]) -> IndexMap<&'static str, i64> {
    let mut map = IndexMap::new();
    map.insert("a", 0);
    map.insert("b", 1);
    map.insert("c", 2);
    map
}

fn integration_values_return_array(_: &mut [ZVal]) -> EBox<ZArray> {
    let mut arr = ZArray::new();
    arr.insert("a", ZVal::from(1));
    arr.insert("b", ZVal::from("foo"));
    arr
}

fn integration_values_return_object(_: &mut [ZVal]) -> EBox<Object<()>> {
    let mut object = Object::new_by_std_class();
    object.set_property("foo", ZVal::from("bar"));
    object
}

fn integration_values_return_option_i64_some(_: &mut [ZVal]) -> Option<i64> {
    Some(64)
}

fn integration_values_return_option_i64_none(_: &mut [ZVal]) -> Option<i64> {
    None
}

fn integration_values_return_result_string_ok(_: &mut [ZVal]) -> phper::Result<String> {
    Ok("foo".to_string())
}

fn integration_values_return_result_string_err(_: &mut [ZVal]) -> phper::Result<()> {
    Err(phper::Error::other("a zhe"))
}

fn integration_values_return_val(_: &mut [ZVal]) -> ZVal {
    ZVal::from("foo")
}
