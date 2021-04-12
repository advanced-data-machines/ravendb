use std::fmt;

#[derive(Debug)]
pub enum RavenError{
    HttpError,   // DB communication errors
    ParseError,  // errors converting typed structs T => Person etc.
                 // ??
}

impl std::error::Error for RavenError {}

impl fmt::Display for RavenError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            RavenError::HttpError => write!(f, "Http(s) Error"),
            RavenError::ParseError => write!(f, "Parse Error")
        }
    }
}

impl From<reqwest::Error> for RavenError {
    fn from(_: reqwest::Error) -> Self {
        RavenError::HttpError
    }
}