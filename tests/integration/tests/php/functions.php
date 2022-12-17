<?php

// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.


require_once __DIR__ . '/_common.php';

integrate_functions_call();

assert_eq(integrate_functions_call_callable(function () { return 1 + 1; }), 2);
assert_eq(integrate_functions_call_callable(function ($a, $b) { return $a + $b; }, 1, 2), 3);
assert_eq(integrate_functions_call_callable("addslashes", "Is your name O'Reilly?"), "Is your name O\'Reilly?");
assert_eq(integrate_functions_call_callable(array(new Exception("something wrong"), "getMessage")), "something wrong");
assert_throw(function() {
    integrate_functions_call_callable(function () { throw new RuntimeException("oh no", 403); });
}, "RuntimeException", 403, "oh no");

assert_throw("integrate_functions_exception_guard", "ErrorException", 0, "other io error");
