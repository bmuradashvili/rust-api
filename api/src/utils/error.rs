use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::Responder;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use jsonwebtoken::errors::{Error as JWTError, ErrorKind};

#[derive(Debug)]
pub enum Error {
    NotFound,
    AuthorizationHeaderMissing,
    AuthorizationTokenExpired,
    AuthorizationTokenInvalid,
    InternalServerError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => f.write_str("NotFound"),
            Error::AuthorizationHeaderMissing => f.write_str("NoAuthorizationHeader"),
            Error::AuthorizationTokenExpired => f.write_str("AuthorizationTokenExpired"),
            Error::AuthorizationTokenInvalid => f.write_str("AuthorizationTokenInvalid"),
            Error::InternalServerError => f.write_str("InternalServerError"),
        }
    }
}


impl From<DieselError> for Error {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => Error::NotFound,
            _ => Error::InternalServerError,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound => "Record not found",
            Error::InternalServerError => "Internal server error",
            _ => "Error"
        }
    }
}

impl From<JWTError> for Error {
    fn from(e: JWTError) -> Self {
        match *e.kind() {
            ErrorKind::InvalidToken => Error::AuthorizationTokenInvalid,
            ErrorKind::ExpiredSignature => Error::AuthorizationTokenExpired,
            _ => Error::AuthorizationHeaderMissing,
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'r> {
        match self {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}
