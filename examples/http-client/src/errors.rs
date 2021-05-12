use phper::{
    classes::{ClassEntry, DynamicClass},
    errors::{Error::ClassNotFound, Throwable},
};

const EXCEPTION_CLASS_NAME: &'static str = "HttpClient\\HttpClientException";

#[derive(thiserror::Error, Debug)]
pub enum HttpClientError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

impl Throwable for HttpClientError {
    fn class_entry(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap()
    }
}

pub fn make_exception_class() -> DynamicClass<()> {
    let exception_class = DynamicClass::new(EXCEPTION_CLASS_NAME);
    exception_class
}
