use phper::{
    arrays::ZArray,
    classes::{ClassEntity, StateClass, Visibility},
};
use std::convert::Infallible;

pub type RequestClass = StateClass<()>;

pub const HTTP_REQUEST_CLASS_NAME: &str = "HttpServer\\HttpRequest";

/// Register the class `HttpServer\HttpRequest` by `ClassEntity`.
pub fn make_request_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(HTTP_REQUEST_CLASS_NAME);

    // Register the http headers field with public visibility.
    class.add_property("headers", Visibility::Public, ());

    // Register the http body field with public visibility.
    class.add_property("data", Visibility::Public, ());

    // Register the constructor method with public visibility, initialize the
    // headers with empty array.
    class.add_method("__construct", Visibility::Public, |this, _arguments| {
        this.set_property("headers", ZArray::new());
        Ok::<_, Infallible>(())
    });

    class
}
