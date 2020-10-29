pub type PHPerResult<T> = Result<T, PHPerError>;

#[derive(thiserror::Error, Debug)]
pub enum PHPerError {}
