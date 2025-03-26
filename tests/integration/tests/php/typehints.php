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
    // <method>, <expected typehint>, <is nullable>, <is required>
    ['testString', 'string', false, true],
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

    ['testMixed', 'mixed', false, true],

    ['testCallable', 'callable', false, true],
    ['testCallableOptional', 'callable', false, false],
    ['testCallableNullable', 'callable', true, true],

    ['testObject', 'object', false, true],
    ['testObjectOptional', 'object', false, false],
    ['testObjectNullable', 'object', true, true],

    ['testIterable', 'iterable', false, true],
    ['testIterableOptional', 'iterable', false, false],
    ['testIterableNullable', 'iterable', true, true],

    ['testNull', 'null', true, true],

    ['testClassEntry', 'class_name', false, true],
    ['testClassEntryOptional', 'class_name', false, false],
    ['testClassEntryNullable', 'class_name', true, true],
];

// typehints
echo 'Testing argument typehints' . PHP_EOL;
$cls = new \IntegrationTest\TypeHints\ArgumentTypeHintTest();
$reflection = new ReflectionClass($cls);
foreach ($argumentTypehintProvider as $input) {
    echo(sprintf("%s..", $input[0]));
    $reflectionMethod = $reflection->getMethod($input[0]);
    $params = $reflectionMethod->getParameters();

    assert_eq(1, count($params), 'has 1 param');
    $param = $params[0];
    $type = $param->getType();
    if (!in_array($input[1], ['mixed'])) {
        assert_eq($input[1], (string)$type->getName(), sprintf('%s has typehint type', $input[0]));
        assert_eq($input[2], $type->allowsNull(), sprintf('%s allows null', $input[0]));
    }
    assert_eq($input[3], !$param->isOptional(), sprintf('%s is optional', $input[0]));
    echo "PASS" . PHP_EOL;
}

// return typehints
$returnTypehintProvider = [
    // <method>, <expected typehint>, <is nullable>
    ['returnNull', 'null', true],
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
    ['returnMixed', 'mixed', true],
    ['returnNever', 'never', false],
    ['returnVoid', 'void', false],
    ['returnClassEntry', 'class_name', false],
    ['returnClassEntryNullable', 'class_name', true],
];
echo PHP_EOL . 'Testing return typehints' . PHP_EOL;
$cls = new \IntegrationTest\TypeHints\ReturnTypeHintTest();
$reflection = new ReflectionClass($cls);
foreach ($returnTypehintProvider as $input) {
    echo(sprintf("%s..", $input[0]));
    $reflectionMethod = $reflection->getMethod($input[0]);
    $return = $reflectionMethod->getReturnType();
    assert_eq($input[1], $return->getName(), sprintf('%s has typehint type', $input[0]));
    assert_eq($input[2], $return->allowsNull(), sprintf('%s allows null', $input[0]));
    echo 'PASS' . PHP_EOL;
}

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