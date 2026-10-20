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

$foo = 100;
integrate_test_reference($foo, $bar);

assert_eq($foo, 200);
assert_eq($bar, "hello");

// Class types are only encoded in internal arginfo from 8.0.
if (PHP_VERSION_ID >= 80000) {
    $param = (new ReflectionFunction('integrate_test_reference_class_type'))->getParameters()[0];
    assert_true($param->isPassedByReference());

    $obj = new stdClass();
    integrate_test_reference_class_type($obj);
    assert_eq($obj, "replaced");

    assert_true((new ReflectionFunction('integrate_test_reference_return_class_type'))->returnsReference());
}
