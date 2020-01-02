use crate::config::Config;

/// A command line interface command
pub trait Command: Sized {
    type Error;
    fn apply(self, config: Config) -> Result<(), Self::Error>;
    fn handle_error(err: Self::Error);
    fn call(self, config: Config) {
        match self.apply(config) {
            Ok(_) => {}
            Err(err) => Self::handle_error(err),
        }
    }
}
