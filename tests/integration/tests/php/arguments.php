<?php

// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.


require_once __DIR__ . '/_common.php';

if (PHP_VERSION_ID >= 70100) {
    $argumentCountErrorName = "ArgumentCountError";
} else {
    $argumentCountErrorName = "TypeError";
}

assert_eq(integrate_arguments_null(null), null);

assert_throw(function () { integrate_arguments_null(); }, $argumentCountErrorName, 0, "integrate_arguments_null(): expects at least 1 parameter(s), 0 given");
assert_throw(function () { integrate_arguments_null(1); }, "TypeError", 0, "type error: must be of type null, int given");

assert_eq(integrate_arguments_long(1, 2), 3);
assert_eq(integrate_arguments_long(1, "2"), 3);
assert_throw(function () { integrate_arguments_long("1", "2"); }, "TypeError", 0, "type error: must be of type int, string given");

assert_eq(integrate_arguments_double(1.0), 1.0);
assert_throw(function () { integrate_arguments_double(1); }, "TypeError", 0, "type error: must be of type float, int given");

assert_eq(integrate_arguments_string("hello", "world"), "hello, world");
assert_eq(integrate_arguments_string("hello", 123), "hello, 123");
assert_throw(function () { integrate_arguments_string(1, 2); }, "TypeError", 0, "type error: must be of type string, int given");

assert_eq(integrate_arguments_array(["a" => 1]), ["a" => 1, "foo" => "bar"]);
assert_throw(function () { integrate_arguments_array(null); }, "TypeError", 0, "type error: must be of type array, null given");

$obj = new stdClass();
$obj->a = 1;
assert_object(integrate_arguments_object($obj), "stdClass", ["a" => 1, "foo" => "bar"]);
assert_throw(function () { integrate_arguments_object(1); }, "TypeError", 0, "type error: must be of type object, int given");

assert_throw(function () { integrate_arguments_optional(); }, $argumentCountErrorName, 0, "integrate_arguments_optional(): expects at least 1 parameter(s), 0 given");
assert_eq(integrate_arguments_optional("foo"), "foo: false");
assert_eq(integrate_arguments_optional("foo", true), "foo: true");
assert_eq(integrate_arguments_optional("foo", true, "bar"), "foo: true");
