<?php

require_once __DIR__ . '/_common.php';

integrate_objects_new_drop();
integrate_objects_get_set();
integrate_objects_set_val();
integrate_objects_call();
integrate_objects_to_ref_owned(new stdClass());
integrate_objects_to_ref_clone(new stdClass());
integrate_objects_set_props();

$a = new IntegrationTest\Objects\A();
assert_throw(function () use ($a) { $a2 = clone $a; }, "Error", 0, "Trying to clone an uncloneable object of class IntegrationTest\\Objects\\A");

$b = new IntegrationTest\Objects\B();
$b2 = clone $b;
$b2->incr();
assert_eq($b->get(), 123456);
assert_eq($b2->get(), 123457);

class B2 extends IntegrationTest\Objects\B {}
$b2 = new B2();
$b22 = clone $b2;
assert_eq($b22->get(), 123456);
