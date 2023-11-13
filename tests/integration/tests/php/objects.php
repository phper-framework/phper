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
