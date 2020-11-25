<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$mc = new MiniCurl("http://httpbin.org/ip");
var_dump($mc);

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new Exception("left != right,\n left: {$left},\n right: {$right};");
    }
}