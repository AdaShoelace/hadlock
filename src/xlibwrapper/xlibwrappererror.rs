use std::error;
use std::fmt;

use super::core::*;

pub type Result<T> = std::result::Result<T, XlibWrapperError>;

#[derive(Debug, Clone)]
pub enum XlibWrapperError {
    DisplayConnectionError,
    GetGeometryError,
    BadValueError,
    BadWindowError,
    UnknownError,
}


impl error::Error for XlibWrapperError {
    fn description(&self) -> &str {
        match *self {
            XlibWrapperError::DisplayConnectionError => "Failed to connect to display server!",
            XlibWrapperError::GetGeometryError => "An error occured when fetching geometry for a Drawable",
            XlibWrapperError::BadValueError => "A value false outside the accepted range",
            XlibWrapperError::BadWindowError => "A value for a Window argument does not name a defined Window",
            XlibWrapperError::UnknownError => "Something went wrong",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl fmt::Display for XlibWrapperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            XlibWrapperError::DisplayConnectionError => format!("{:?}", self),
            XlibWrapperError::GetGeometryError => format!("{:?}", self),
            XlibWrapperError::BadValueError => format!("{:?}", self),
            XlibWrapperError::BadWindowError => format!("{:?}", self),
            XlibWrapperError::UnknownError => format!("{:?}", self),
        };
        write!(f, "{}", description)
    }
}



