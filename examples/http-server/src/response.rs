use crate::errors::HttpServerError;
use hyper::{header::HeaderName, http::HeaderValue, Body, Response};
use phper::{
    classes::{DynamicClass, Visibility},
    functions::Argument,
};

pub const HTTP_RESPONSE_CLASS_NAME: &'static str = "HttpServer\\HttpResponse";

pub fn make_response_class() -> DynamicClass<Response<Body>> {
    let mut class = DynamicClass::new_with_default(HTTP_RESPONSE_CLASS_NAME);

    class.add_method(
        "header",
        Visibility::Public,
        |this, arguments| {
            let response: &mut Response<Body> = this.as_mut_state();
            response.headers_mut().insert(
                HeaderName::from_bytes(arguments[0].as_string()?.as_bytes())?,
                HeaderValue::from_bytes(arguments[1].as_string()?.as_bytes())?,
            );
            Ok::<_, HttpServerError>(())
        },
        vec![Argument::by_val("data")],
    );

    class.add_method(
        "end",
        Visibility::Public,
        |this, arguments| {
            let response: &mut Response<Body> = this.as_mut_state();
            *response.body_mut() = arguments[0].as_string()?.into();
            Ok::<_, phper::Error>(())
        },
        vec![Argument::by_val("data")],
    );

    class
}
