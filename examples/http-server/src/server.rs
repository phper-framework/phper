// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{errors::HttpServerError, request::new_request_object, response::new_response_object};
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    routing::any,
    Router, Server,
};
use hyper::body;
use phper::{
    alloc::ToRefOwned,
    classes::{ClassEntity, Visibility},
    functions::Argument,
    values::ZVal,
};
use std::{cell::RefCell, collections::HashMap, mem::take, net::SocketAddr};
use tokio::runtime::{self};

const HTTP_SERVER_CLASS_NAME: &str = "HttpServer\\HttpServer";

thread_local! {
    static ON_REQUEST_HANDLERS: RefCell<HashMap<u32, ZVal>> = Default::default();
}

pub fn make_server_class() -> ClassEntity<Option<SocketAddr>> {
    let mut class = ClassEntity::new_with_default_state_constructor(HTTP_SERVER_CLASS_NAME);

    class.add_property("host", Visibility::Private, "127.0.0.1");
    class.add_property("port", Visibility::Private, 8080);

    class
        .add_method("__construct", Visibility::Public, |this, arguments| {
            let host = arguments[0].expect_z_str()?;
            let port = arguments[1].expect_long()?;

            this.set_property("host", host.to_owned());
            this.set_property("port", port);

            let addr = format!("{}:{}", host.to_str()?, port)
                .parse::<SocketAddr>()
                .map_err(|e| HttpServerError(Box::new(e)))?;
            *this.as_mut_state() = Some(addr);

            Ok::<_, phper::Error>(())
        })
        .arguments([Argument::by_val("host"), Argument::by_val("port")]);

    class
        .add_method("onRequest", Visibility::Public, |this, arguments| {
            ON_REQUEST_HANDLERS.with(|handlers| {
                handlers
                    .borrow_mut()
                    .insert(this.handle(), arguments[0].clone());
            });
            Ok::<_, phper::Error>(())
        })
        .argument(Argument::by_val("handle"));

    class.add_method("start", Visibility::Public, |this, _| {
        let addr = take(this.as_mut_state()).unwrap();
        let handle = this.handle();

        let fut = async move {
            let app = Router::new().route(
                "/",
                any(move |req: Request<Body>| async move {
                    match (async move {
                        let (parts, body) = req.into_parts();
                        let body = body::to_bytes(body)
                            .await
                            .map_err(|e| HttpServerError(Box::new(e)))?;

                        let mut request = new_request_object()?;

                        let request_headers =
                            request.get_mut_property("headers").expect_mut_z_arr()?;
                        for (key, value) in parts.headers {
                            if let Some(key) = key {
                                request_headers.insert(key.as_str(), value.as_bytes());
                            }
                        }
                        request.set_property("data", &*body);
                        let request_val = ZVal::from(request);

                        let mut response = new_response_object()?;
                        let response_val = ZVal::from(response.to_ref_owned());

                        ON_REQUEST_HANDLERS.with(|handlers| {
                            let mut handlers = handlers.borrow_mut();
                            let handler = handlers.get_mut(&handle).unwrap();

                            handler.call([request_val, response_val])?;

                            let response =
                                take(unsafe { response.as_mut_state::<Response<Body>>() });
                            Ok::<Response<Body>, phper::Error>(response)
                        })
                    })
                    .await
                    {
                        Ok(response) => response,
                        Err(e) => {
                            let mut response = Response::new("".into());
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = e.to_string().into();
                            response
                        }
                    }
                }),
            );

            let server = Server::bind(&addr).serve(app.into_make_service());

            server.await
        };

        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut)
            .map_err(|e| HttpServerError(Box::new(e)))?;

        Ok::<_, phper::Error>(())
    });

    class
}
