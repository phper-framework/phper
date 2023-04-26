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

$response = $client->get("https://example.com/")->send();
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
