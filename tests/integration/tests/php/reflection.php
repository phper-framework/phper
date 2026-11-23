<?php

require_once __DIR__ . '/_common.php';

$ext = new ReflectionExtension("integration");
assert_eq($ext->getName(), "integration");
assert_eq($ext->getVersion(), "0.0.0");
