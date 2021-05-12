use reqwest::blocking::{Client, ClientBuilder};
use std::time::Duration;
use phper::classes::DynamicClass;
use anyhow::Context;
use crate::errors::HttpClientError;
use phper::functions::Argument;

const HTTP_CLIENT_CLASS_NAME: &'static str = "HttpClient\\HttpClient";

pub fn make_client_class() -> DynamicClass<Client> {
    let mut http_client_class = DynamicClass::new_with_constructor(HTTP_CLIENT_CLASS_NAME, || {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(15))
            .build()?;
        Ok::<_, HttpClientError>(client)
    });

    http_client_class.add_method("get", |this, arguments| {
    }, vec![Argument::by_val("url")]);

    http_client_class
}
