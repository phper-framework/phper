use crate::errors::HttpServerError;
use hyper::{
    server::{conn::AddrIncoming, Builder},
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use phper::{
    classes::{DynamicClass, Visibility},
    functions::Argument,
    values::Val,
};
use std::{convert::Infallible, mem::replace, net::SocketAddr};
use tokio::{runtime::Handle};

const HTTP_SERVER_CLASS_NAME: &'static str = "HttpServer\\HttpServer";

pub fn make_server_class() -> DynamicClass<Option<Builder<AddrIncoming>>> {
    let mut class = DynamicClass::new_with_default(HTTP_SERVER_CLASS_NAME);

    class.add_property("host", Visibility::Private, "127.0.0.1");
    class.add_property("port", Visibility::Private, 8080);

    class.add_method(
        "__construct",
        Visibility::Public,
        |this, arguments| {
            let host = arguments[0].as_string()?;
            let port = arguments[1].as_long()?;
            this.set_property("host", Val::new(&*host));
            this.set_property("port", Val::new(port));
            let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
            let builder = Server::bind(&addr);
            *this.as_mut_state() = Some(builder);
            Ok::<_, HttpServerError>(())
        },
        vec![Argument::by_val("host"), Argument::by_val("port")],
    );

    class.add_method(
        "start",
        Visibility::Public,
        |this, _| {
            let builder = replace(this.as_mut_state(), None).unwrap();

            let make_svc =
                make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

            let server = builder.serve(make_svc);
            Handle::current().block_on(server)?;

            Ok::<_, HttpServerError>(())
        },
        vec![],
    );

    class
}

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World!".into()))
}
