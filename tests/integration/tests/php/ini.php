<?php

require_once __DIR__ . '/_common.php';

assert_eq(ini_get("INTEGRATE_INI_TRUE"), '1');
assert_eq(ini_get("INTEGRATE_INI_FALSE"), '0');
assert_eq(ini_get("INTEGRATE_INI_LONG"), '100');
assert_eq(ini_get("INTEGRATE_INI_DOUBLE"), '200');
assert_eq(ini_get("INTEGRATE_INI_STRING"), "something");

integrate_ini_assert();
