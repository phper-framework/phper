<?php

require_once __DIR__ . '/_common.php';

if (PHP_VERSION_ID >= 70100) {
    $argumentCountErrorName = "ArgumentCountError";
} else {
    $argumentCountErrorName = "TypeError";
}

assert_throw(function () { new \IntegrationTest\A(); }, $argumentCountErrorName, 0, "IntegrationTest\\A::__construct(): expects at least 2 parameter(s), 0 given");

$a = new \IntegrationTest\A("foo", 99);
assert_eq($a->speak(), "name: foo, number: 99");

$reflection_class = new ReflectionClass(\IntegrationTest\A::class);

$property_name = $reflection_class->getProperty("name");
assert_true($property_name->isPrivate());
