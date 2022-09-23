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

assert_eq(INTEGRATE_CONST_NULL, null);
assert_eq(INTEGRATE_CONST_TRUE, true);
assert_eq(INTEGRATE_CONST_FALSE, false);
assert_eq(INTEGRATE_CONST_LONG, 100);
assert_eq(INTEGRATE_CONST_DOUBLE, 200.0);
assert_eq(INTEGRATE_CONST_STRING, "something");
assert_eq(INTEGRATE_CONST_BYTES, "something");
