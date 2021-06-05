use phper::classes::{DynamicClass, Visibility};

pub const HTTP_REQUEST_CLASS_NAME: &'static str = "HttpServer\\HttpRequest";

pub fn make_request_class() -> DynamicClass<()> {
    let mut class = DynamicClass::new(HTTP_REQUEST_CLASS_NAME);

    class.add_property("header", Visibility::Public, ());
    class.add_property("server", Visibility::Public, ());
    class.add_property("data", Visibility::Private, ());

    class
}
