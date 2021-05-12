<?php

use HttpClient\HttpClient;

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$client = new HttpClient();
$ip = $client->get("http://httpbin.org/ip");
var_dump($ip);
