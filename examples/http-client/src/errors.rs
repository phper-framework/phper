use phper::{
    classes::{ClassEntry, DynamicClass, StatelessClassEntry},
    errors::Throwable,
};

const EXCEPTION_CLASS_NAME: &'static str = "HttpClient\\HttpClientException";

#[derive(Debug, thiserror::Error, phper::Throwable)]
#[throwable_class(EXCEPTION_CLASS_NAME)]
pub enum HttpClientError {
    #[error(transparent)]
    #[throwable(transparent)]
    Phper(#[from] phper::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("should call '{method_name}()' before call 'body()'")]
    ResponseAfterRead { method_name: String },

    #[error("should not call 'body()' multi time")]
    ResponseHadRead,
}

pub fn make_exception_class() -> DynamicClass<()> {
    let mut exception_class = DynamicClass::new(EXCEPTION_CLASS_NAME);
    exception_class.extends("Exception");
    exception_class
}
