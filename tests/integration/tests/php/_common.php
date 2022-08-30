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

if (!defined('PHP_VERSION_ID')) {
    $version = explode('.', PHP_VERSION);
    define('PHP_VERSION_ID', ($version[0] * 10000 + $version[1] * 100 + $version[2]));
}

function assert_true($value) {
    assert_eq($value, true);
}

function assert_false($value) {
    assert_eq($value, false);
}

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new AssertionError(sprintf("left != right,\n left: %s,\n right: %s", var_export($left, true), var_export($right, true)));
    }
}

function assert_object($object, $expect_class_name, $expect_fields) {
    if (get_class($object) != $expect_class_name) {
        throw new AssertionError(sprintf("expect class `%s`, found `%s`", $expect_class_name, get_class($object)));
    }
    foreach ($expect_fields as $key => $value) {
        if ($object->$key !== $value) {
            throw new AssertionError(sprintf("expect field `%s` value %s, found `%s`", $key, $value, $object->$key));
        }
    }
}

function assert_throw($callable, $expect_exception_class, $expect_exception_code, $expect_exception_message) {
    try {
        $callable();
        throw new AssertionError("`{$expect_exception_message}` not throws");
    } catch (Throwable $e) {
        if (get_class($e) != $expect_exception_class) {
            throw new AssertionError(sprintf("expect exception class `%s`, found `%s`", $expect_exception_class, get_class($e)));
        }
        if ($e->getCode() != $expect_exception_code) {
            throw new AssertionError(sprintf("expect exception code `%s`, found `%s`", $expect_exception_code, $e->getCode()));
        }
        if ($e->getMessage() != $expect_exception_message) {
            throw new AssertionError(sprintf("expect exception message `%s`, found `%s`", $expect_exception_message, $e->getMessage()));
        }
    }
}

function array_ksort($array) {
    ksort($array);
    return $array;
}
