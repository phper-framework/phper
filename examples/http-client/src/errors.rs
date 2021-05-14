use phper::{
    classes::{ClassEntry, DynamicClass, StatelessClassEntry},
    errors::Throwable,
};

const EXCEPTION_CLASS_NAME: &'static str = "HttpClient\\HttpClientException";

#[derive(thiserror::Error, Debug)]
pub enum HttpClientError {
    #[error(transparent)]
    Phper(#[from] phper::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

impl Throwable for HttpClientError {
    fn class_entry(&self) -> &StatelessClassEntry {
        ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap()
    }
}

pub fn make_exception_class() -> DynamicClass<()> {
    let mut exception_class = DynamicClass::new(EXCEPTION_CLASS_NAME);
    exception_class.extends("Exception");
    exception_class
}
