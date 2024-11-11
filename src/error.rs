use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The string specified does not contain a valid square")]
    InvalidSquare,
}
