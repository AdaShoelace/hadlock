use std::error;
use std::fmt;

use super::core::*;

pub type Result<T> = std::result::Result<T, XlibWrapperError>;

#[derive(Debug, Clone)]
pub enum XlibWrapperError {
    DisplayConnection,
    GetGeometry,
    BadValue,
    BadWindow,
    BadMatch,
    BadAlloc,
    BadAtom,
    Unknown,
}


impl error::Error for XlibWrapperError {
    fn description(&self) -> &str {
        match *self {
            XlibWrapperError::DisplayConnection => "Failed to connect to display server!",
            XlibWrapperError::GetGeometry => "An error occured when fetching geometry for a Drawable",
            XlibWrapperError::BadValue => "A value false outside the accepted range",
            XlibWrapperError::BadWindow => "A value for a Window argument does not name a defined Window",
            XlibWrapperError::BadMatch => "BadMatchError",
            XlibWrapperError::BadAlloc => "Server failed to allocate memory",
            XlibWrapperError::BadAtom => "No Atom defined by that value",
            XlibWrapperError::Unknown => "Something went wrong",
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
            XlibWrapperError::DisplayConnection => format!("{:?}", self),
            XlibWrapperError::GetGeometry => format!("{:?}", self),
            XlibWrapperError::BadValue => format!("{:?}", self),
            XlibWrapperError::BadWindow => format!("{:?}", self),
            XlibWrapperError::BadMatch => format!("{:?}", self),
            XlibWrapperError::BadAlloc => format!("{:?}", self),
            XlibWrapperError::BadAtom => format!("{:?}", self),
            XlibWrapperError::Unknown => format!("{:?}", self),
        };
        write!(f, "{}", description)
    }
}



