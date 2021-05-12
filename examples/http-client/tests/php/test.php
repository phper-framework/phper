<?php

use HttpClient\HttpClient;

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$client = new HttpClient();
var_dump($client);
