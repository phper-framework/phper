use crate::{
    errors::HttpClientError,
    response::{ReadiedResponse, RESPONSE_CLASS_NAME},
};
use anyhow::Context;
use phper::{classes::DynamicClass, functions::Argument, objects::Object};
use reqwest::{
    blocking::{Client, ClientBuilder},
    Response,
};
use std::{mem::MaybeUninit, time::Duration};

const HTTP_CLIENT_CLASS_NAME: &'static str = "HttpClient\\HttpClient";

pub fn make_client_class() -> DynamicClass<Client> {
    let mut class = DynamicClass::new_with_constructor(HTTP_CLIENT_CLASS_NAME, || {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(15))
            .build()?;
        Ok::<_, HttpClientError>(client)
    });

    class.add_method(
        "get",
        |this, arguments| {
            let url = arguments[0].as_string()?;
            let client = this.as_state();
            let response = client.get(url).send()?;

            let readied_response = ReadiedResponse {
                status: response.status(),
                remote_addr: response.remote_addr(),
                headers: response.headers().clone(),
                body: response.bytes()?,
            };

            let mut response_object =
                Object::<Option<ReadiedResponse>>::new_by_class_name(RESPONSE_CLASS_NAME)
                    .map_err(phper::Error::ClassNotFound)?;
            *response_object.as_mut_state() = Some(readied_response);

            Ok::<_, HttpClientError>(response_object)
        },
        vec![Argument::by_val("url")],
    );

    class
}
