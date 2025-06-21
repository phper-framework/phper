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


use HttpServer\HttpServer;

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$server = new HttpServer("127.0.0.1", 9010);
$server->onRequest(function ($request, $response) {
    echo "HEADERS:\n";
    foreach ($request->headers as $key => $value) {
        echo "$key => $value\n";
    }
    echo "\nBODY:\n{$request->data}\n\n";

    $response->header('Content-Type', 'text/plain');
    $response->header('X-Foo', 'Bar');
    $response->end("Hello World\n");
});

echo "Listening http://127.0.0.1:9010\n\n";

$server->start();
