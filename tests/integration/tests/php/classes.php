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

// Test bound_class
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
assert_false($doSomethings->isStatic());

$myStaticMethod = $interface->getMethod("myStaticMethod");
assert_true($myStaticMethod->isPublic());
assert_true($myStaticMethod->isAbstract());
assert_true($myStaticMethod->isStatic());

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

// Test final classes. Extending one is a compile error rather than a catchable
// throwable, so assert the flag by reflection as Zend/tests/enum/final.phpt
// does; the non-final class is the control.
assert_true((new ReflectionClass(IntegrationTest\FinalClass::class))->isFinal());
assert_false((new ReflectionClass(IntegrationTest\NonFinalClass::class))->isFinal());

// Test typed properties
$typedPropertyProvider = [
    // <property>, <type>, <allows null>, <has default>, <default>
    ['boolProp', 'bool', false, true, true],
    ['intProp', 'int', false, true, 42],
    ['floatProp', 'float', false, true, 1.5],
    ['stringProp', 'string', false, false, null],
    ['defaultStringProp', 'string', false, true, 'foo'],
    ['nullableStringProp', '?string', true, true, null],
    ['arrayProp', 'array', false, false, null],
    ['objectProp', 'object', false, false, null],
    ['iterableProp', '?iterable', true, true, null],
    ['mixedProp', 'mixed', true, false, null],
    ['classProp', 'IntegrationTest\A', false, false, null],
    ['nullableClassProp', '?IntegrationTest\A', true, true, null],
];
$reflection = new ReflectionClass(IntegrationTest\TypedProperties::class);
foreach ($typedPropertyProvider as list($name, $type, $allowsNull, $hasDefault, $default)) {
    $property = $reflection->getProperty($name);
    if (PHP_VERSION_ID >= 80000) {
        assert_eq((string) $property->getType(), $type, $name);
        assert_eq($property->getType()->allowsNull(), $allowsNull, $name);
        assert_eq($property->hasDefaultValue(), $hasDefault, $name);
        assert_eq($property->getDefaultValue(), $default, $name);
    } else {
        // Untyped below 8.0, keeping the default or falling back to null.
        assert_eq($reflection->getDefaultProperties()[$name], $default, $name);
    }
}

if (PHP_VERSION_ID >= 80000) {
    $typed = new IntegrationTest\TypedProperties();
    assert_eq($typed->intProp, 42);
    assert_throw(function () use ($typed) { $typed->intProp = "forty-two"; }, "TypeError", 0, 'Cannot assign string to property IntegrationTest\TypedProperties::$intProp of type int');
    assert_throw(function () use ($typed) { return $typed->stringProp; }, "Error", 0, 'Typed property IntegrationTest\TypedProperties::$stringProp must not be accessed before initialization');
    $typed->stringProp = "foo";
    assert_eq($typed->stringProp, "foo");

    $typed->classProp = new IntegrationTest\A("foo", 1);
    assert_throw(function () use ($typed) { $typed->classProp = new stdClass(); }, "TypeError", 0, 'Cannot assign stdClass to property IntegrationTest\TypedProperties::$classProp of type IntegrationTest\A');

    $typed->iterableProp = [1, 2];
    $typed->iterableProp = (function () { yield 1; })();
    // From 8.2 errors name the type iterable is lowered to, as for a userland
    // property; reflection still says ?iterable.
    $iterableType = php_at_least('8.2') ? 'Traversable|array|null' : '?iterable';
    assert_throw(function () use ($typed) { $typed->iterableProp = 1; }, "TypeError", 0, 'Cannot assign int to property IntegrationTest\TypedProperties::$iterableProp of type ' . $iterableType);
}

// Test readonly properties, modelled on
// Zend/tests/readonly_props/readonly_modification.phpt. Below 8.1 they are
// plain typed properties.
$reflection = new ReflectionClass(IntegrationTest\ReadonlyProperties::class);
if (PHP_VERSION_ID >= 80100) {
    assert_true($reflection->getProperty('value')->isReadOnly());
    assert_true($reflection->getProperty('nullableClass')->isReadOnly());
    assert_false($reflection->getProperty('mutable')->isReadOnly());
}
if (PHP_VERSION_ID >= 80000) {
    assert_eq((string) $reflection->getProperty('value')->getType(), 'int');
    assert_eq((string) $reflection->getProperty('nullableClass')->getType(), '?IntegrationTest\A');
    assert_false($reflection->getProperty('value')->hasDefaultValue());
}

$readonly = new IntegrationTest\ReadonlyProperties();
$readonly->init(1);
assert_eq($readonly->value, 1);
if (PHP_VERSION_ID >= 80100) {
    // Messages differ across versions (8.4 made public readonly
    // protected(set)), so only the class is asserted.
    $writes = [
        'second write from the handler' => function () use ($readonly) { $readonly->init(2); },
        'write from userland' => function () use ($readonly) { $readonly->value = 2; },
        'initialisation from userland' => function () {
            $fresh = new IntegrationTest\ReadonlyProperties();
            $fresh->value = 1;
        },
    ];
    foreach ($writes as $case => $write) {
        try {
            $write();
            throw new AssertionError("$case did not throw");
        } catch (Error $e) {
            assert_eq(get_class($e), 'Error', $case);
        }
    }
    assert_eq($readonly->value, 1);
} else {
    $readonly->init(2);
    $readonly->value = 3;
    assert_eq($readonly->value, 3);
}

// Test readonly classes, modelled on Zend/tests/readonly_classes/. Their
// properties are readonly from 8.2; dynamic properties are refused from 8.0.
// ReadonlyProperties is the control: readonly properties, but not the class.
$reflection = new ReflectionClass(IntegrationTest\ReadonlyClass::class);
if (PHP_VERSION_ID >= 80200) {
    assert_true($reflection->isReadOnly());
    assert_false((new ReflectionClass(IntegrationTest\ReadonlyProperties::class))->isReadOnly());
    assert_true($reflection->getProperty('value')->isReadOnly());
}
$readonly = new IntegrationTest\ReadonlyClass();
$readonly->init(1);
if (PHP_VERSION_ID >= 80200) {
    assert_throw(function () use ($readonly) { $readonly->init(2); }, "Error", 0, 'Cannot modify readonly property IntegrationTest\ReadonlyClass::$value');
} else {
    $readonly->init(2);
}
if (PHP_VERSION_ID >= 80000) {
    assert_throw(function () use ($readonly) { $readonly->dynamic = 1; }, "Error", 0, 'Cannot create dynamic property IntegrationTest\ReadonlyClass::$dynamic');
}

// Test co-dependent classes
$b = IntegrationTest\Dependency\A::createB();
assert_true($b instanceof IntegrationTest\Dependency\B);
$a = IntegrationTest\Dependency\B::createA();
assert_true($a instanceof IntegrationTest\Dependency\A);
