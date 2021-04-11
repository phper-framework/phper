use std::{ffi::FromBytesWithNulError, io, str::Utf8Error};

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
