use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("The string specified does not contain a valid square")]
    InvalidSquare,

    #[error("The string specified does not contain a valid rank")]
    InvalidRank,

    #[error("The string specified does not contain a valid file")]
    InvalidFile,

    #[error("Cannot move pinned piece!")]
    CannotMovePinned,

    #[error("No piece found on square!")]
    NoPieceOnSquare,
}
