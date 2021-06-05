<?php

use HttpServer\HttpServer;

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$server = new HttpServer("127.0.0.1", 9000);
$server->onRequest(function ($request, $response) {
    $response->header('Content-Type', 'text/plain');
    $response->end("Hello World\n");
});
$server->start();
