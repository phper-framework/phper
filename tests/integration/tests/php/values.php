<?php

require_once __DIR__ . '/_common.php';

assert_eq(integration_values_return_null(), null);
assert_eq(integration_values_return_i32(), 32);
assert_eq(integration_values_return_i64(), 64);
