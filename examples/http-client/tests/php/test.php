<?php

use HttpClient\HttpClientBuilder;
use HttpClient\HttpClient;
use HttpClient\HttpClientException;

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$client_builder = new HttpClientBuilder();
$client_builder->timeout(15000);
$client_builder->cookie_store(true);
$client = $client_builder->build();

$request_builder = $client->get("https://httpbin.org/ip");
$response = $request_builder->send();
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
