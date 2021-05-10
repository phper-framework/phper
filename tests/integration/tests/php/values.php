<?php

require_once __DIR__ . '/_common.php';

assert_eq(integration_values_return_null(), null);
assert_eq(integration_values_return_true(), true);
assert_eq(integration_values_return_false(), false);
assert_eq(integration_values_return_i32(), 32);
assert_eq(integration_values_return_u32(), 32);
assert_eq(integration_values_return_i64(), 64);
assert_eq(integration_values_return_f64(), 64.0);
assert_eq(integration_values_return_str(), "foo");
assert_eq(integration_values_return_string(), "foo");
assert_eq(integration_values_return_i64_vec(), [0, 1, 2]);
assert_eq(integration_values_return_string_vec(), ["a", "b", "c"]);
assert_eq(array_ksort(integration_values_return_i64_map()), ["a" => 0, "b" => 1, "c" => 2]);
assert_eq(array_ksort(integration_values_return_string_map()), ["a" => "x", "b" => "y", "c" => "z"]);
assert_eq(integration_values_return_i64_index_map(), ["a" => 0, "b" => 1, "c" => 2]);
assert_eq(integration_values_return_array(), ["a" => 1, "b" => "foo"]);
assert_object(integration_values_return_object(), "stdClass", ["foo" => "bar"]);
assert_eq(integration_values_return_option_i64_some(), 64);
assert_eq(integration_values_return_option_i64_none(), null);
assert_eq(integration_values_return_result_string_ok(), "foo");
assert_throw("integration_values_return_result_string_err", "ErrorException", 0, "a zhe");
assert_eq(integration_values_return_val(), "foo");
