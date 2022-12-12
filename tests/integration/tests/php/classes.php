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


$foo = new \IntegrationTest\Foo();
