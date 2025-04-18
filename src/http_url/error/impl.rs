use super::*;
use crate::*;

impl StdError for HttpUrlError {}

impl Display for HttpUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpUrlError::InvalidUrl => write!(f, "Invalid URL"),
        }
    }
}
