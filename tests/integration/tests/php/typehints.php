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

// class implements module-provided interface
$b = new \IntegrationTest\TypeHints\B();
$foo = $b->createFoo();
$foo->setValue('foobar');
$value = $foo->getValue();
assert_eq($value, 'foobar');

$argumentTypehintProvider = [
    // <method>, <expected typehint>, <is nullable>, <is required>, <(optional)min php version>
    ['testString', 'string', false, true, '7.1'],
    ['testStringOptional', 'string', false, false],
    ['testStringNullable', 'string', true, true],

    ['testInt', 'int', false, true],
    ['testIntOptional', 'int', false, false],
    ['testIntNullable', 'int', true, true],

    ['testBool', 'bool', false, true],
    ['testBoolOptional', 'bool', false, false],
    ['testBoolNullable', 'bool', true, true],

    ['testFloat', 'float', false, true],
    ['testFloatOptional', 'float', false, false],
    ['testFloatNullable', 'float', true, true],

    ['testArray', 'array', false, true],
    ['testArrayOptional', 'array', false, false],
    ['testArrayNullable', 'array', true, true],

    ['testMixed', 'mixed', true, true, '8.0'],

    ['testCallable', 'callable', false, true],
    ['testCallableOptional', 'callable', false, false],
    ['testCallableNullable', 'callable', true, true],

    ['testObject', 'object', false, true, '7.2'],
    ['testObjectOptional', 'object', false, false, '7.2'],
    ['testObjectNullable', 'object', true, true, '7.2'],

    ['testIterable', 'iterable', false, true, '7.1'],
    ['testIterableOptional', 'iterable', false, false, '7.1'],
    ['testIterableNullable', 'iterable', true, true, '7.1'],

    ['testNull', 'null', true, true, '8.2'],

    ['testClassEntry', 'IntegrationTest\\TypeHints\\IFoo', false, true, '8.0'],
    ['testClassEntryOptional', 'IntegrationTest\\TypeHints\\IFoo', false, false, '8.0'],
    ['testClassEntryNullable', 'IntegrationTest\\TypeHints\\IFoo', true, true, '8.0'],
];

// typehints
echo 'Testing argument typehints' . PHP_EOL;
$cls = new \IntegrationTest\TypeHints\ArgumentTypeHintTest();
$reflection = new ReflectionClass($cls);
foreach ($argumentTypehintProvider as $input) {
    echo(sprintf("%s..", $input[0]));
    if (array_key_exists(4, $input) && !php_at_least($input[4])) {
        echo sprintf("SKIPPED requires at least PHP %s", $input[4]) . PHP_EOL;
        continue;
    }
    $reflectionMethod = $reflection->getMethod($input[0]);
    $params = $reflectionMethod->getParameters();

    assert_eq(1, count($params), 'has 1 param');
    $param = $params[0];
    $type = $param->getType();
    if (PHP_VERSION_ID >= 70200) {
        assert_eq($input[1], (string)$type->getName(), sprintf('%s has typehint type', $input[0]));
        assert_eq($input[2], $type->allowsNull(), sprintf('%s allows null', $input[0]));
        assert_eq($input[3], !$param->isOptional(), sprintf('%s is optional', $input[0]));
    } else {
        //ReflectionType::getName doesn't exist until 7.1
        assert_eq($input[1], (string)$type);
    }
    echo "PASS" . PHP_EOL;
}

// return typehints
$returnTypehintProvider = [
    // <method>, <expected typehint>, <is nullable>, <(optional)min php version>
    ['returnNull', 'null', true, '8.2'],
    ['returnBool', 'bool', false],
    ['returnBoolNullable', 'bool', true],
    ['returnInt', 'int', false],
    ['returnIntNullable', 'int', true],
    ['returnFloat', 'float', false],
    ['returnFloatNullable', 'float', true],
    ['returnString', 'string', false],
    ['returnStringNullable', 'string', true],
    ['returnArray', 'array', false],
    ['returnArrayNullable', 'array', true],
    ['returnObject', 'object', false],
    ['returnObjectNullable', 'object', true],
    ['returnCallable', 'callable', false],
    ['returnCallableNullable', 'callable', true],
    ['returnIterable', 'iterable', false],
    ['returnIterableNullable', 'iterable', true],
    ['returnMixed', 'mixed', true, '8.0'],
    ['returnNever', 'never', false, '8.1'],
    ['returnVoid', 'void', false],
    ['returnClassEntry', 'IntegrationTest\\TypeHints\\IFoo', false, '8.0'],
    ['returnClassEntryNullable', 'IntegrationTest\\TypeHints\\IFoo', true, '8.0'],
];
echo PHP_EOL . 'Testing return typehints' . PHP_EOL;
$cls = new \IntegrationTest\TypeHints\ReturnTypeHintTest();
$reflection = new ReflectionClass($cls);
foreach ($returnTypehintProvider as $input) {
    echo(sprintf("%s..", $input[0]));
    if (array_key_exists(3, $input) && !php_at_least($input[3])) {
        echo sprintf("SKIPPED requires at least PHP %s", $input[3]) . PHP_EOL;
        continue;
    }
    $reflectionMethod = $reflection->getMethod($input[0]);
    $return = $reflectionMethod->getReturnType();
    if ($input[1] !== 'never' && PHP_VERSION_ID > 70200) {
        assert_eq($input[1], $return->getName(), sprintf('%s has typehint type', $input[0]));
        assert_eq($input[2], $return->allowsNull(), sprintf('%s allows null', $input[0]));
    }
    echo 'PASS' . PHP_EOL;
}

if (PHP_VERSION_ID > 70100) {
    // test class entry type-hints with an instance
    $foo = new class implements \IntegrationTest\TypeHints\IFoo {
        private $value;
        public function getValue(): string {
            return $this->value;
        }
        public function setValue($value): void {
            $this->value = $value;
        }
    };

    $foo->setValue('hello');
    assert_eq('hello', $foo->getValue());

    $handler = new \IntegrationTest\TypeHints\FooHandler();
    assert_eq($foo, $handler->handle($foo));
}

$argumentDefaultValueProvider = [
    // <method>, <expected default value>, <(optional) min php version>
    ['stringDefault', 'foobarbaz', 'string', '8.0'],
    ['stringConstantDefault', PHP_VERSION, 'string', '8.0'],
    ['boolDefaultTrue', true, 'boolean', '8.0'],
    ['boolDefaultFalse', false, 'boolean', '8.0'],
    ['intDefault', 42, 'integer', '8.0'],
    ['floatDefault', 3.14159, 'double', '8.0'],
    ['arrayDefault', ['a' => 'b'], 'array', '8.0'],
    ['iterableDefault', [0 => 1], 'array', '8.0'],
    ['mixedDefault', 999, 'integer', '8.0'],
];

echo PHP_EOL . 'Testing argument default values' . PHP_EOL;
$cls = new IntegrationTest\TypeHints\ArgumentDefaultValueTest();
$reflection = new ReflectionClass($cls);
foreach ($argumentDefaultValueProvider as $input) {
    echo(sprintf("%s..", $input[0]));
    if (array_key_exists(3, $input) && !php_at_least($input[3])) {
        echo sprintf("SKIPPED requires at least PHP %s", $input[3]) . PHP_EOL;
        continue;
    }
    $reflectionMethod = $reflection->getMethod($input[0]);
    $params = $reflectionMethod->getParameters();
    $param = $params[0];
    assert_true($param->isDefaultValueAvailable());
    $default = $param->getDefaultValue();
    assert_eq($input[1], $default, sprintf('%s has expected default value', $input[0]));
    assert_eq($input[2], gettype($default), sprintf('%s default value has expected type', $input[0]));
    echo "PASS" . PHP_EOL;
}

$expectedArgs = [
    // <arg name>, <type>, <default value>
    ['s', 'string', 'foobarbaz'],
    ['i', 'int', 42],
    ['f', 'float', 7.89],
    ['b', 'bool', true],
    ['a', 'array', ['a'=>'b']],
    ['m', 'mixed', 1.23],
    ['ce', 'Stringable'], //default value not supported for ClassEntry
];
if (PHP_VERSION_ID >= 80000) {
    echo PHP_EOL . 'Testing function typehints' . PHP_EOL;
    $reflection = new ReflectionFunction('integration_function_typehints');
    $params = $reflection->getParameters();
    foreach ($expectedArgs as $i => $input) {
        echo(sprintf("argument %d..", $i));
        assert_eq($input[0], $params[$i]->getName(), sprintf('argument %d has correct name', $i));
        assert_eq($input[1], $params[$i]->getType()->getName(), sprintf('argument %d has correct type', $i));
        if (array_key_exists(2, $input)) {
            assert_eq($input[2], $params[$i]->getDefaultValue(), sprintf('argument %d has correct default value', $i));
        }
        echo "PASS" . PHP_EOL;
    }
    assert_eq('void', $reflection->getReturnType()->getName(), 'integration_function_typehints return type is void');
}

//invoke type-hinted functions to exercise handlers
echo PHP_EOL . 'Testing return type-hinted function invocation' . PHP_EOL;
assert_true(integration_function_return_bool());
assert_eq(42, integration_function_return_int());
assert_eq(3.14, integration_function_return_float());
assert_eq('phper', integration_function_return_string());
assert_eq(array(), integration_function_return_array());
assert_eq(1.23, integration_function_return_mixed());

//invoke type-hinted class methods to exercise handlers
echo PHP_EOL . 'Testing return type-hinted method invocation' . PHP_EOL;
$cls = new \IntegrationTest\TypeHints\ReturnTypeHintTest();
assert_eq(true, $cls->returnBool(), 'returnBool');
assert_eq(null, $cls->returnBoolNullable(), 'returnBoolNullable');
assert_eq(42, $cls->returnInt(), 'returnInt');
assert_eq(null, $cls->returnIntNullable(), 'returnIntNullable');
assert_eq(3.14, $cls->returnFloat(), 'returnFloat');
assert_eq(null, $cls->returnFloatNullable(), 'returnFloatNullable');
assert_eq('phper', $cls->returnString(), 'returnString');
assert_eq(null, $cls->returnStringNullable(), 'returnStringNullable');
assert_eq(array(), $cls->returnArray(), 'returnArray');
assert_eq(null, $cls->returnArrayNullable(), 'returnArrayNullable');
