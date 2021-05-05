<?php

require_once __DIR__ . '/_common.php';

assert_eq(integrate_arguments_null(null), null);
assert_throw(function () { integrate_arguments_null(); }, "ArgumentCountError", 0, "integrate_arguments_null(): expects at least 1 parameter(s), 0 given");
assert_throw(function () { integrate_arguments_null(1); }, "TypeError", 0, "type error: must be of type null, int given");
