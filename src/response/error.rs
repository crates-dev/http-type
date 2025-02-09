use http_constant::*;
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

use crate::Response;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ResponseError(String),
    HasSendResponse(Response),
    Unknown,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponseError(data) => write!(f, "Response Error{}{}", COLON_SPACE, data),
            Self::HasSendResponse(data) => write!(f, "Response Error{}{:?}", COLON_SPACE, data),
            Self::Unknown => write!(f, "{}", "Unknown"),
        }
    }
}
