<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

assert_eq((new HelloClass())->sayHello(), "Hello, world!");

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new Exception("left != right,\n left: {$left},\n right: {$right};");
    }
}