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


ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

assert_eq(complex_say_hello("world"), "Hello, world!\n");

try {
    complex_throw_exception();
} catch (ErrorException $e) {
    assert_eq($e->getMessage(), "I am sorry");
}

assert_eq(complex_get_all_ini(), [
    "complex.enable" => false,
    "complex.description" => "hello world.",
]);

$foo = new FooClass();
assert_eq($foo->getFoo(), 100);

$foo->setFoo("Hello");
assert_eq($foo->getFoo(), "Hello");

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new AssertionError(sprintf("left != right,\n left: %s,\n right: %s", var_export($left, true), var_export($right, true)));
    }
}
