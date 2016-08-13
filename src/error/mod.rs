use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;

pub use err::Error;
pub use hyper::Error as HttpError;
pub use hyper::error::Result as HttpResult;
use hyper::status::StatusCode;

#[derive(Debug)]
pub enum ErrorTiming {
    AtNetwork,
    AtRequest,
    AtResponse,
}

#[derive(Debug)]
pub struct ApiError {
    pub error: Box<Error + Send>,
    pub timing: ErrorTiming,
}

impl ApiError {
    pub fn new<E: Error>(e: E, timing: ErrorTiming) -> ApiError {
        ApiError {
            error: Box::new(e),
            timing: timing,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&*self.error, f)
    }
}

impl StdError for ApiError {
    fn description(&self) -> &str {
        self.error.description()
    }

    fn cause(&self) -> Option<&StdError> {
        self.error.cause()
    }
}

#[derive(Debug)]
pub enum ResponseError {
    UnacceptableStatusCode(StatusCode),
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ResponseError::UnacceptableStatusCode(ref code) => {
                write!(f, "Unacceptable Status Code: {}", code)
            }
        }
    }
}

impl StdError for ResponseError {
    fn description(&self) -> &str {
        match *self {
            ResponseError::UnacceptableStatusCode(code) => "Unacceptable Status code",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        return None;
    }
}
