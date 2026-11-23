use crate::{errors::HttpClientError, response::ResponseClass};
use phper::classes::{ClassEntity, StateClass, Visibility};
use reqwest::blocking::RequestBuilder;
use std::{convert::Infallible, mem::take};

pub type RequestBuilderClass = StateClass<Option<RequestBuilder>>;

pub const REQUEST_BUILDER_CLASS_NAME: &str = "HttpClient\\RequestBuilder";

pub fn make_request_builder_class(
    response_class: ResponseClass,
) -> ClassEntity<Option<RequestBuilder>> {
    let mut class = ClassEntity::<Option<RequestBuilder>>::new_with_default_state_constructor(
        REQUEST_BUILDER_CLASS_NAME,
    );

    class.add_method("__construct", Visibility::Private, |_, _| {
        Ok::<_, Infallible>(())
    });

    class.add_method("send", Visibility::Public, move |this, _arguments| {
        let state = take(this.as_mut_state());
        let response = state.unwrap().send().map_err(HttpClientError::Reqwest)?;
        let mut object = response_class.new_object([])?;
        *object.as_mut_state() = Some(response);
        Ok::<_, phper::Error>(object)
    });

    class
}
