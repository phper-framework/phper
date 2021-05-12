use crate::errors::HttpClientError;
use anyhow::Context;
use phper::{classes::DynamicClass, functions::Argument};
use reqwest::blocking::{Client, ClientBuilder};
use std::time::Duration;

const HTTP_CLIENT_CLASS_NAME: &'static str = "HttpClient\\HttpClient";

pub fn make_client_class() -> DynamicClass<Client> {
    let mut http_client_class = DynamicClass::new_with_constructor(HTTP_CLIENT_CLASS_NAME, || {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(15))
            .build()?;
        Ok::<_, HttpClientError>(client)
    });

    http_client_class.add_method(
        "get",
        |this, arguments| {
            let url = arguments[0].as_string()?;
            let client = this.as_state();
            let response = client.get(url).send().unwrap();
            let body = response.text().unwrap();
            Ok::<_, phper::Error>(body)
        },
        vec![Argument::by_val("url")],
    );

    http_client_class
}
