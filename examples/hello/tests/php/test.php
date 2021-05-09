<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

assert_eq(hello_say_hello("world"), "Hello, world!\n");

try {
    hello_throw_exception();
} catch (ErrorException $e) {
    assert_eq($e->getMessage(), "I am sorry");
}

assert_eq(hello_get_all_ini(), [
    "hello.enable" => false,
    "hello.description" => "hello world.",
]);

$foo = new FooClass();
// TODO change '100' to 100.
assert_eq($foo->getFoo(), '100');

$foo->setFoo("Hello");
assert_eq($foo->getFoo(), "Hello");

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new Exception(sprintf("left != right,\n left: %s,\n right: %s", var_export($left, true), var_export($right, true)));
    }
}