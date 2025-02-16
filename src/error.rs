use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum Error {
    #[error("The string specified does not contain a valid square")]
    InvalidSquare,

    #[error("The string specified does not contain a valid rank")]
    InvalidRank,

    #[error("The string specified does not contain a valid file")]
    InvalidFile,

    #[error("The string specified does not contain a valid move")]
    InvalidMove,

    #[error("Cannot move pinned piece!")]
    CannotMovePinned,

    #[error("Cannot make move when game has ended!")]
    GameEnded,

    #[error("No piece found on square!")]
    NoPieceOnSquare,
}
