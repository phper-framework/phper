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
assert_eq(integration_values_return_array(), ["a" => 1, "b" => "foo"]);
