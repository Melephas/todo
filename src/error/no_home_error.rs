use std::fmt::{Display, Formatter, Result};
use thiserror::Error;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Error)]
pub struct NoHomeError;

impl Display for NoHomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Unable to find home directory")
    }
}