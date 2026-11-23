<?php

require_once __DIR__ . '/_common.php';

assert_throw("integrate_throw_boxed", "ErrorException", 0, "What's wrong with you?");
assert_throw("integrate_throw_object", "Exception", 403, "Forbidden");
