<?php

// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.


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

// Test bind_class
$a_instance = IntegrationTest\A::newInstance();
assert_true($a_instance instanceof IntegrationTest\A);
assert_eq($a_instance->speak(), "name: default, number: 100");

// Test registering class;
$foo = new \IntegrationTest\Foo();

// Test implementation of Iterator interface.
$tmp_arr = [];
foreach ($foo as $key => $value) {
    $tmp_arr[] = [$key, $value];
}
assert_eq($tmp_arr, [[0, 'Current: 0'], [1, 'Current: 1'], [2, 'Current: 2']]);

// Test implementation of ArrayAccess interface.
assert_eq($foo[10], null);
$foo[10] = "10";
assert_eq($foo[10], "10");
unset($foo[10]);
assert_eq($foo[10], null);

// Test registering interface;
assert_true(interface_exists("\\IntegrationTest\\IBar"));

$interface = new ReflectionClass("\\IntegrationTest\\IBar");

assert_true($interface->isInterface());
assert_true($interface->isInternal());

assert_true($interface->implementsInterface("ArrayAccess"));
assert_true($interface->implementsInterface("Iterator"));

$doSomethings = $interface->getMethod("doSomethings");
assert_true($doSomethings->isPublic());
assert_true($doSomethings->isAbstract());

// Test get or set static properties.
assert_eq(IntegrationTest\PropsHolder::$foo, "bar");

assert_eq(IntegrationTest\PropsHolder::getFoo1(), 12345);
$pre_foo1 = IntegrationTest\PropsHolder::setFoo1("baz");
assert_eq($pre_foo1, 12345);
assert_eq(IntegrationTest\PropsHolder::getFoo1(), "baz");

// Test php class extends from phper registered class.
class Foo2 extends IntegrationTest\Foo {}
$foo2 = new Foo2();
assert_eq($foo2->current(), 'Current: 0');

// Test Stringable implementation.
if (PHP_VERSION_ID >= 80000) {
    assert_eq(((string) (new IntegrationTest\FooString())), 'string');
}

// Test class constants
assert_eq('foo', IntegrationTest\A::CST_STRING);
assert_eq(null, IntegrationTest\A::CST_NULL);
assert_true(true, IntegrationTest\A::CST_TRUE);
assert_false(false, IntegrationTest\A::CST_FALSE);
assert_eq(100, IntegrationTest\A::CST_INT);
assert_eq(10.0, IntegrationTest\A::CST_FLOAT);

// Test interface constants
assert_true(interface_exists(IntegrationTest\IConstants::class));
assert_eq('foo', IntegrationTest\IConstants::CST_STRING);
assert_eq(null, IntegrationTest\IConstants::CST_NULL);
assert_true(IntegrationTest\IConstants::CST_TRUE);
assert_false(IntegrationTest\IConstants::CST_FALSE);
assert_eq(100, IntegrationTest\IConstants::CST_INT);
assert_eq(10.0, IntegrationTest\IConstants::CST_FLOAT);

// Test module class extends module class
$bar = new \IntegrationTest\BarExtendsFoo; //Bar should extend Foo
$reflection = new ReflectionClass($bar);
assert_true($reflection->isSubclassOf(IntegrationTest\Foo::class));