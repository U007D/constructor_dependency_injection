use crate::consts::msg;
use thiserror::Error;
use std::convert::Infallible;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}", msg::SAMPLE_ERROR)]
    SampleError,
}

impl From<std::convert::Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
