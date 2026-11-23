<?php

require_once __DIR__ . '/_common.php';

$foo = 100;
integrate_test_reference($foo, $bar);

assert_eq($foo, 200);
assert_eq($bar, "hello");

// Class types are only encoded in internal arginfo from 8.0.
if (PHP_VERSION_ID >= 80000) {
    $param = (new ReflectionFunction('integrate_test_reference_class_type'))->getParameters()[0];
    assert_true($param->isPassedByReference());

    $obj = new stdClass();
    integrate_test_reference_class_type($obj);
    assert_eq($obj, "replaced");

    assert_true((new ReflectionFunction('integrate_test_reference_return_class_type'))->returnsReference());
}
