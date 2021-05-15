use phper::classes::DynamicClass;
use reqwest::blocking::{Request, RequestBuilder};

pub const REQUEST_BUILDER_CLASS_NAME: &'static str = "HttpClient\\RequestBuilder";

pub fn make_request_builder_class() -> DynamicClass<Option<RequestBuilder>> {
    let mut class = DynamicClass::new_with_none(REQUEST_BUILDER_CLASS_NAME);

    class
}
