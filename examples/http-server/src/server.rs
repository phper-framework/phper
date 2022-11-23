// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{
    errors::HttpServerError, request::HTTP_REQUEST_CLASS_NAME, response::HTTP_RESPONSE_CLASS_NAME,
    utils::replace_and_get,
};
use hyper::{
    server::{conn::AddrIncoming, Builder},
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use phper::{
    alloc::{EBox, RefClone, ToRefOwned},
    classes::{ClassEntry, StatefulClass, Visibility},
    errors::Error::Throw,
    functions::Argument,
    values::ZVal,
};
use std::{
    convert::Infallible,
    mem::replace,
    net::SocketAddr,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};
use tokio::runtime::Handle;

const HTTP_SERVER_CLASS_NAME: &str = "HttpServer\\HttpServer";

pub fn make_server_class() -> StatefulClass<Option<Builder<AddrIncoming>>> {
    let mut class = StatefulClass::<Option<Builder<AddrIncoming>>>::new_with_default_state(
        HTTP_SERVER_CLASS_NAME,
    );

    class.add_property("host", Visibility::Private, "127.0.0.1");
    class.add_property("port", Visibility::Private, 8080);
    class.add_property("onRequestHandle", Visibility::Private, ());

    class.add_constructor(
        Visibility::Public,
        |this, arguments| {
            let host = arguments[0].expect_z_str()?;
            let port = arguments[1].expect_long()?;
            this.set_property("host", host.to_owned());
            this.set_property("port", port);
            let addr = format!("{}:{}", host.to_str()?, port).parse::<SocketAddr>()?;
            let builder = Server::bind(&addr);
            *this.as_mut_state() = Some(builder);
            Ok::<_, HttpServerError>(())
        },
        vec![Argument::by_val("host"), Argument::by_val("port")],
    );

    class.add_method(
        "onRequest",
        Visibility::Public,
        |this, arguments| {
            this.set_property("onRequestHandle", arguments[0].clone());
            Ok::<_, phper::Error>(())
        },
        vec![Argument::by_val("handle")],
    );

    class.add_method(
        "start",
        Visibility::Public,
        |this, _| {
            static HANDLE: AtomicPtr<ZVal> = AtomicPtr::new(null_mut());

            let builder = replace(this.as_mut_state(), None).unwrap();
            let handle = EBox::new(this.get_mut_property("onRequestHandle").ref_clone());
            HANDLE.store(EBox::into_raw(handle), Ordering::SeqCst);

            let make_svc = make_service_fn(move |_conn| async move {
                Ok::<_, Infallible>(service_fn(move |_: Request<Body>| async move {
                    match async move {
                        let handle = unsafe { HANDLE.load(Ordering::SeqCst).as_mut().unwrap() };

                        let request =
                            ClassEntry::from_globals(HTTP_REQUEST_CLASS_NAME)?.new_object([])?;
                        let request = ZVal::from(request);

                        let mut response =
                            ClassEntry::from_globals(HTTP_RESPONSE_CLASS_NAME)?.new_object([])?;
                        let response_val = response.to_ref_owned();
                        let response_val = ZVal::from(response_val);

                        match handle.call([request, response_val]) {
                            Err(Throw(ex)) => {
                                let state = unsafe { response.as_mut_state::<Response<Body>>() };
                                *state.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                *state.body_mut() = ex.to_string().into();
                            }
                            Err(e) => return Err(e.into()),
                            _ => {}
                        }

                        let response = replace_and_get(unsafe { response.as_mut_state() });
                        Ok::<Response<Body>, HttpServerError>(response)
                    }
                    .await
                    {
                        Ok(response) => Ok::<Response<Body>, Infallible>(response),
                        Err(e) => {
                            let mut response = Response::new("".into());
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = e.to_string().into();
                            Ok::<Response<Body>, Infallible>(response)
                        }
                    }
                }))
            });

            let server = builder.serve(make_svc);
            Handle::current().block_on(server)?;

            Ok::<_, HttpServerError>(())
        },
        vec![],
    );

    class
}
