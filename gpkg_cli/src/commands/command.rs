use crate::config::Config;
use miette::Diagnostic;
use std::error::Error;

/// A command line interface command
pub trait Command: Sized {
    type Error: Error + Diagnostic;
    fn apply(self, config: Config) -> Result<(), Self::Error>;
}
