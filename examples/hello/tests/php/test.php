<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

assert_eq(hello_say_hello("world"), "Hello, world!\n");

assert_eq(class_exists("PHPerException"), true);

try {
    hello_throw_exception();
} catch (PHPerException $e) {
    assert_eq($e->getMessage(), "I am sorry");
}

assert_eq(hello_get_all_ini(), [
    "hello.enable" => true,
    "hello.description" => "hello world.",
]);

$foo = new FooClass();
assert_eq($foo->getFoo(), 100);

$foo->setFoo("Hello");
assert_eq($foo->getFoo(), "Hello");

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new Exception("left != right,\n left: {$left},\n right: {$right};");
    }
}