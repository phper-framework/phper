use reqwest::blocking::{Client, ClientBuilder};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> reqwest::Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(15))
            .build()?;
        Ok(Self { client })
    }
}
