use crate::{errors::HttpClientError, response::RESPONSE_CLASS_NAME, utils::replace_and_get};
use phper::{
    classes::{ClassEntry, DynamicClass, Visibility},
    objects::Object,
};
use reqwest::blocking::{RequestBuilder, Response};

pub const REQUEST_BUILDER_CLASS_NAME: &'static str = "HttpClient\\RequestBuilder";

pub fn make_request_builder_class() -> DynamicClass<Option<RequestBuilder>> {
    let mut class = DynamicClass::new_with_default(REQUEST_BUILDER_CLASS_NAME);

    class.add_method(
        "__construct",
        Visibility::Private,
        |_: &mut Object<Option<RequestBuilder>>, _| {},
        vec![],
    );

    class.add_method(
        "send",
        Visibility::Public,
        |this, _arguments| {
            let state = this.as_mut_state();
            let response = replace_and_get(state, |builder| builder.unwrap().send())?;
            let mut object =
                ClassEntry::<Option<Response>>::from_globals(RESPONSE_CLASS_NAME)?.init_object()?;
            *object.as_mut_state() = Some(response);
            Ok::<_, HttpClientError>(object)
        },
        vec![],
    );

    class
}
