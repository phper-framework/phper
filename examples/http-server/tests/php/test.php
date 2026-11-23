<?php

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
