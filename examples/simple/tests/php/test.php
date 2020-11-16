<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

assert_eq(get_extension_funcs('simple'), ["test_simple"]);
assert_eq(test_simple("aaa", "bbb"), "a = aaa, a_len = 3, b = bbb, b_len = 3");
assert_eq((new MyClass())->foo("foo-"), "foo-3");

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new Exception("left != right,\n left: {$left},\n right: {$right};");
    }
}