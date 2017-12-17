use std;

#[derive(Debug)]
pub enum Error {
    #[doc(hidden)]
    InvalidPatch { description: String },

    #[doc(hidden)]
    Io {
        description: String,
        cause: std::io::Error,
    },
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidPatch { .. } => "The patch is invalid",
            Error::Io { .. } => "An I/O error occurred",
        }
    }
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::InvalidPatch { .. } => None,
            Error::Io { ref cause, .. } => Some(cause),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let d = {
            use std::error::Error;
            self.description()
        };
        match *self {
            Error::InvalidPatch { ref description } => write!(f, "{}: {}", d, description),
            Error::Io {
                ref cause,
                ref description,
            } => write!(f, "{}: {}: {}", d, description, cause),
        }
    }
}

#[cfg(test)]
mod tests {

    use std;
    use super::Error;

    #[test]
    fn error_impl_error_trait() {
        fn impl_error<T: std::error::Error>() {}
        impl_error::<Error>();
    }
}
