use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpnCalcError {
    #[error("parse stack item: {0}")]
    ParseStackItem(String),
    #[error("not enough arguments")]
    NotEnoughArguments,
    #[error("invalid argument {0}")]
    InvalidArgument(String),
    #[error("IO error: {0}")]
    StdIoError(#[from] std::io::Error),
    #[error("invalid units {0}")]
    InvalidUnits(String),
}
