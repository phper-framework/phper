<?php

use HttpClient\HttpClient;
use HttpClient\HttpClientException;

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$client = new HttpClient();

$resp = $client->get("https://httpbin.org/ip");
var_dump([
    "status" => $resp->status(),
    "body" => $resp->body(),
]);

try {
    $client->get("file:///");
    throw new AssertionError("no throw exception");
} catch (HttpClientException $e) {
}
