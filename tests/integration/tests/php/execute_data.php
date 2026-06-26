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

// 2 required + 2 optional, 2 args provided → fill both optionals
assert_eq(materialize_missing_two_optionals(1, "world"), "1, world, 42, hello");

// 2 required + 2 optional, 3 args provided → skip 1st default, fill 2nd
assert_eq(materialize_missing_two_optionals(1, "world", 10), "1, world, 10, hello");

// 2 required + 2 optional, all 4 args provided → no-op
assert_eq(materialize_missing_two_optionals(1, "world", 10, "foo"), "1, world, 10, foo");

// no optional params → no-op
assert_eq(materialize_missing_no_optionals(1, "world"), "1, world");

// defaults exceed declared param count
assert_throw(
    function () { materialize_missing_exceed_error(); },
    "TypeError",
    0,
    "call arg index 2 out of bounds: must be in [0, 2) (declared_len = 2)"
);

// not enough defaults
assert_throw(
    function () { materialize_missing_insufficient_error(); },
    "TypeError",
    0,
    "call arg index 0 out of bounds: must be in [0, 2) (declared_len = 2)"
);
