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

// PHP 8.1+ is required to use enums feature
if (PHP_VERSION_ID < 80100) {
    echo "PHP 8.1+ required for enum tests\n";
    exit(0);
}

// Test pure enum creation and usage
assert_true(enum_exists('IntegrationTest\PureEnum'), 'PureEnum should exist');

// Check enum constants
assert_eq(IntegrationTest\PureEnum::VERSION, '1.0.0', 'PureEnum VERSION constant should be 1.0.0');

// Test static methods
assert_eq(IntegrationTest\PureEnum::getDescription(), 'Pure enum implementation', 'PureEnum::getDescription() should return proper value');

// Test direct access to enum members
assert_eq((IntegrationTest\PureEnum::ONE)->name, 'ONE');
assert_eq((IntegrationTest\PureEnum::TWO)->name, 'TWO');
assert_eq((IntegrationTest\PureEnum::THREE)->name, 'THREE');

// Test int-backed enum
assert_true(enum_exists('IntegrationTest\IntEnum'), 'IntEnum should exist');
assert_eq((IntegrationTest\IntEnum::LOW)->value, 1, 'IntEnum::LOW value should be 1');
assert_eq((IntegrationTest\IntEnum::MEDIUM)->value, 5, 'IntEnum::MEDIUM value should be 5');
assert_eq((IntegrationTest\IntEnum::HIGH)->value, 10, 'IntEnum::HIGH value should be 10');

// Test string-backed enum
assert_true(enum_exists('IntegrationTest\StringEnum'), 'StringEnum should exist');
assert_eq((IntegrationTest\StringEnum::RED)->value, 'FF0000', 'StringEnum::RED value should be FF0000');
assert_eq((IntegrationTest\StringEnum::GREEN)->value, '00FF00', 'StringEnum::GREEN value should be 00FF00');
assert_eq((IntegrationTest\StringEnum::BLUE)->value, '0000FF', 'StringEnum::BLUE value should be 0000FF');

// Test reflection API
$reflection = new ReflectionEnum(IntegrationTest\StringEnum::class);
assert_true($reflection->isBacked(), 'StringEnum should be a backed enum');
assert_true($reflection->hasCase('RED'), 'StringEnum should have a RED case');
assert_true($reflection->hasCase('GREEN'), 'StringEnum should have a GREEN case');
assert_true($reflection->hasCase('BLUE'), 'StringEnum should have a BLUE case');
