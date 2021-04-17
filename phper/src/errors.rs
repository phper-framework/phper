use crate::{classes::ClassEntity, modules::read_global_module, Error::Other};
use anyhow::anyhow;
use std::{error, ffi::FromBytesWithNulError, io, str::Utf8Error};

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    FromBytesWithNul(#[from] FromBytesWithNulError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn other(message: impl ToString) -> Self {
        let message = message.to_string();
        Other(anyhow!(message))
    }
}

pub trait Throwable: error::Error {
    fn class_entity(&self) -> *const ClassEntity;
    fn code(&self) -> u64;
}

pub(crate) const EXCEPTION_CLASS_NAME: &'static str = "PHPerException";

impl Throwable for Error {
    fn class_entity(&self) -> *const ClassEntity {
        read_global_module(|module| {
            module
                .class_entities
                .get(EXCEPTION_CLASS_NAME)
                .expect("Must be called after module init") as *const _
        })
    }

    fn code(&self) -> u64 {
        500
    }
}
