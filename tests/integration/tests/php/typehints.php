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

$typehintProvider = [
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
];

// typehints
echo 'Testing TypeHints' . PHP_EOL;
$c = new \IntegrationTest\TypeHints\C();
$reflection = new ReflectionClass($c);
foreach ($typehintProvider as $input) {
    echo(sprintf("%s..", $input[0]));
    $reflectionMethod = $reflection->getMethod($input[0]);
    $params = $reflectionMethod->getParameters();

    assert_eq(1, count($params), 'has 1 param');
    $param = $params[0];
    $type = $param->getType();
    if (!in_array($input[1], ['mixed'])) {
        assert_eq($input[1], (string)$type->getName(), 'has typehint type');
        assert_eq($input[2], $type->allowsNull(), 'allows null');
    }
    assert_eq($input[3], !$param->isOptional(), 'is optional');
    echo "PASS" . PHP_EOL;
}
