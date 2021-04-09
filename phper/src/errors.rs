use std::str::Utf8Error;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error("unknown value type `{0}`")]
    UnKnownValueType(u32),
}
