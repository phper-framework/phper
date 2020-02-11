<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$functions = get_extension_funcs('simple');
var_dump($functions);
var_dump(test_simple());
