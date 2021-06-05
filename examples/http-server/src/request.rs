use crate::errors::HttpServerError;
use phper::{
    classes::{DynamicClass, Visibility},
    values::Val,
};

pub const HTTP_REQUEST_CLASS_NAME: &'static str = "HttpServer\\HttpRequest";

pub fn make_request_class() -> DynamicClass<()> {
    let mut class = DynamicClass::new(HTTP_REQUEST_CLASS_NAME);

    class.add_property("header", Visibility::Public, ());
    class.add_property("server", Visibility::Public, ());
    class.add_property("data", Visibility::Private, ());

    class.add_method(
        "getData",
        Visibility::Public,
        |this, _| {
            if this.get_property("data").get_type().is_null() {
                this.set_property("data", Val::new("Some data here"));
            }
            Ok::<_, HttpServerError>(this.duplicate_property("data"))
        },
        vec![],
    );

    class
}
