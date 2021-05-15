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

    #[error("should call '{method_name}()' before call 'body()'")]
    ResponseAfterRead { method_name: String },

    #[error("should not call 'body()' multi time")]
    ResponseHadRead,
}

impl Throwable for HttpClientError {
    fn class_entry(&self) -> &StatelessClassEntry {
        match self {
            HttpClientError::Phper(e) => e.class_entry(),
            _ => ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap(),
        }
    }
}

pub fn make_exception_class() -> DynamicClass<()> {
    let mut exception_class = DynamicClass::new(EXCEPTION_CLASS_NAME);
    exception_class.extends("Exception");
    exception_class
}
