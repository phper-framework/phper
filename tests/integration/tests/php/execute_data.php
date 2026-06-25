<?php

// Copyright (c) 2026 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.


require_once __DIR__ . '/_common.php';

// Test 1: fill all missing arguments with defaults
assert_eq(materialize_missing_fill(), "42, hello");

// Test 2: provide all arguments, no filling needed
assert_eq(materialize_missing_noop(1, "world"), "2, 1, world");

// Test 3: partial fill - only second arg is missing
assert_eq(materialize_missing_partial("hello"), "hello, 42");

// Test 4: exceed declared args causes TypeError
assert_throw(
    function () { materialize_missing_exceed_error(); },
    "TypeError",
    0,
    "call arg index 2 out of bounds: must be in [0, 2) (declared_len = 2)"
);

// Test 5: insufficient defaults causes TypeError
assert_throw(
    function () { materialize_missing_insufficient_error(); },
    "TypeError",
    0,
    "call arg index 0 out of bounds: must be in [0, 2) (declared_len = 2)"
);
