<?php

require_once __DIR__ . '/_common.php';

assert_eq(INTEGRATE_CONST_NULL, null);
assert_eq(INTEGRATE_CONST_TRUE, true);
assert_eq(INTEGRATE_CONST_FALSE, false);
assert_eq(INTEGRATE_CONST_LONG, 100);
assert_eq(INTEGRATE_CONST_DOUBLE, 200.0);
assert_eq(INTEGRATE_CONST_STRING, "something");
assert_eq(INTEGRATE_CONST_BYTES, "something");
