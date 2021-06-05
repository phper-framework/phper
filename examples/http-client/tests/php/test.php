<?php

use HttpClient\HttpClientBuilder;
use HttpClient\HttpClient;
use HttpClient\HttpClientException;

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$client = (new HttpClientBuilder())
    ->timeout(15000)
    ->cookie_store(true)
    ->build();

$response = $client->get("https://httpbin.org/ip")->send();
var_dump([
    "status" => $response->status(),
    "headers" => $response->headers(),
    "body" => $response->body(),
]);

try {
    $client->get("file:///")->send();
    throw new AssertionError("no throw exception");
} catch (HttpClientException $e) {
}
