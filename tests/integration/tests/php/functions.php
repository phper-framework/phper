<?php

require_once __DIR__ . '/_common.php';

integrate_functions_call();

assert_eq(integrate_functions_call_callable(function () { return 1 + 1; }), 2);
assert_eq(integrate_functions_call_callable(function ($a, $b) { return $a + $b; }, 1, 2), 3);
assert_eq(integrate_functions_call_callable("addslashes", "Is your name O'Reilly?"), "Is your name O\'Reilly?");
