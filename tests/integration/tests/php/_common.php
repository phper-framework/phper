<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new AssertionError(sprintf("left != right,\n left: %s,\n right: %s", var_export($left, true), var_export($right, true)));
    }
}

function assert_throw($callable, $expect_exception_class, $expect_exception_code, $expect_exception_message) {
    try {
        $callable();
        throw new AssertionError("`{$expect_exception_message}` not throws");
    } catch (Exception $e) {
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