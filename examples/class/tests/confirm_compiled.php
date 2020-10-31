<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

var_dump((new MyClass())->foo("foo-"));
