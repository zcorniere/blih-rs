use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BlihErr {
    InvalidRequest,
    InvalidUrl,
    RequestFailed,
    NoTokenProvided,
}

impl fmt::Display for BlihErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlihErr::InvalidRequest  => write!(f, "Invalid request"),
            BlihErr::InvalidUrl      => write!(f, "Invalid Url"),
            BlihErr::RequestFailed   => write!(f, "Request Failed"),
            BlihErr::NoTokenProvided => write!(f, "No token was provided"),
        }
    }
}
