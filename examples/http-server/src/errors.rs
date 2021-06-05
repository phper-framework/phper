use hyper::header::{InvalidHeaderName, InvalidHeaderValue};
use phper::classes::{ClassEntry, DynamicClass};
use std::net::AddrParseError;

const EXCEPTION_CLASS_NAME: &'static str = "HttpServer\\HttpServerException";

#[derive(Debug, thiserror::Error, phper::Throwable)]
#[throwable_class(EXCEPTION_CLASS_NAME)]
pub enum HttpServerError {
    #[error(transparent)]
    #[throwable(transparent)]
    Phper(#[from] phper::Error),

    #[error(transparent)]
    AddrParse(#[from] AddrParseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    #[error(transparent)]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
}

pub fn make_exception_class() -> DynamicClass<()> {
    let mut exception_class = DynamicClass::new(EXCEPTION_CLASS_NAME);
    exception_class.extends("Exception");
    exception_class
}
