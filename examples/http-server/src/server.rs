// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{errors::HttpServerError, request::RequestClass, response::ResponseClass};
use axum::{
    Router,
    body::{self, Body},
    http::{Request, Response, StatusCode},
    routing::any,
};
use phper::{
    alloc::ToRefOwned,
    classes::{ClassEntity, Visibility},
    functions::Argument,
    values::ZVal,
};
use std::{cell::RefCell, collections::HashMap, error::Error, net::SocketAddr};
use tokio::{net::TcpListener, runtime};

const HTTP_SERVER_CLASS_NAME: &str = "HttpServer\\HttpServer";

// Because PHP is a single threaded model (NTS), so it can hold the global
// variables by thread local.
thread_local! {
    // The map store the on request handlers, indexed by `HttpServer\HttpServer` object id.
    static ON_REQUEST_HANDLERS: RefCell<HashMap<u32, ZVal>> = Default::default();

    static CLASSES_MAP: RefCell<HashMap<u32, (RequestClass, ResponseClass)>> = Default::default();
}

/// Register the class `HttpServer\HttpServer` by `ClassEntity`.
pub fn make_server_class(
    request_class: RequestClass, response_class: ResponseClass,
) -> ClassEntity<()> {
    let mut class = ClassEntity::new(HTTP_SERVER_CLASS_NAME);

    // Register the server host field with public visibility.
    class.add_property("host", Visibility::Private, "127.0.0.1");

    // Register the server port field with public visibility.
    class.add_property("port", Visibility::Private, 8080);

    // Register the constructor method with public visibility, accept host and port
    // arguments, initialize the host and port member properties.
    class
        .add_method("__construct", Visibility::Public, move |this, arguments| {
            let host = arguments[0].expect_z_str()?;
            let port = arguments[1].expect_long()?;

            this.set_property("host", host.to_owned());
            this.set_property("port", port);

            let (request_class, response_class) = (request_class.clone(), response_class.clone());

            CLASSES_MAP.with(move |classes_map| {
                classes_map
                    .borrow_mut()
                    .insert(this.handle(), (request_class, response_class));
            });

            Ok::<_, phper::Error>(())
        })
        .arguments([Argument::new("host"), Argument::new("port")]);

    // Register the onRequest method, with public visibility, insert the handle into
    // global ON_REQUEST_HANDLERS map.
    class
        .add_method("onRequest", Visibility::Public, |this, arguments| {
            ON_REQUEST_HANDLERS.with(|handlers| {
                handlers
                    .borrow_mut()
                    .insert(this.handle(), arguments[0].clone());
            });
            Ok::<_, phper::Error>(())
        })
        .argument(Argument::new("handle"));

    // Register the start method, with public visibility, this method will start and
    // http server, listen on the addr, and block.
    class.add_method("start", Visibility::Public, |this, _| {
        // Get the host and port from properties, and create the SocketAddr.
        let host = this.get_property("host").expect_z_str()?.to_str()?;
        let port = this.get_property("port").expect_long()?;
        let addr = format!("{}:{}", host, port)
            .parse::<SocketAddr>()
            .map_err(HttpServerError::new)?;

        // Get the object id, as key of ON_REQUEST_HANDLERS.
        let handle = this.handle();

        // This is the main loop of the http server, it use axum web framework to
        // achieve the effect of the http server.
        let fut = async move {
            // Simply to handle all routes by "/" path and any method.
            let app = Router::new().route(
                "/",
                any(move |req: Request<Body>| async move {
                    let fut = async move {
                        let (parts, body) = req.into_parts();

                        // Read all request body content.
                        let body = body::to_bytes(body, usize::MAX)
                            .await
                            .map_err(HttpServerError::new)?;

                        CLASSES_MAP.with(|classes_map| {
                            let (request_class, response_class) = &classes_map.borrow()[&handle];

                            // Create PHP `HttpServer\HttpRequest` object.
                            let mut request = request_class.new_object([])?;

                            // Inject headers from Rust request object to PHP request object.
                            let request_headers =
                                request.get_mut_property("headers").expect_mut_z_arr()?;
                            for (key, value) in parts.headers {
                                if let Some(key) = key {
                                    request_headers.insert(key.as_str(), value.as_bytes());
                                }
                            }

                            // Inject body content from Rust request object to PHP request object.
                            request.set_property("data", &*body);

                            let request_val = ZVal::from(request);

                            // Create PHP `HttpServer\HttpResponse` object.
                            let mut response = response_class.new_object([])?;

                            let response_val = ZVal::from(response.to_ref_owned());

                            ON_REQUEST_HANDLERS.with(|handlers| {
                                // Get the on request handlers by object id.
                                let mut handlers = handlers.borrow_mut();
                                let handler = handlers.get_mut(&handle).unwrap();

                                // Call the PHP on request handler.
                                handler.call([request_val, response_val])?;

                                // Get the inner state.
                                let response = response.into_state().unwrap();

                                Ok::<Response<Body>, phper::Error>(response)
                            })
                        })
                    };
                    match fut.await {
                        Ok(response) => response,
                        Err(e) => {
                            // If failed, simply return 500 as http status code, and the error
                            // message as http response body.
                            let mut response = Response::new("".into());
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = e.to_string().into();
                            response
                        }
                    }
                }),
            );

            // Start the http server.
            let listener = TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?;
            Ok::<_, Box<dyn Error>>(())
        };

        // Build the tokio runtime, with the current thread scheduler, block on
        // listening socket addr and handling the server request.
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut)
            .map_err(HttpServerError::new)?;

        Ok::<_, phper::Error>(())
    });

    class
}
