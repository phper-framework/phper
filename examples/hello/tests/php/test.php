<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

assert_eq(get_extension_funcs('hello'), ["say_hello"]);
assert_eq(say_hello("world"), "Hello, world!");

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new Exception("left != right,\n left: {$left},\n right: {$right};");
    }
}