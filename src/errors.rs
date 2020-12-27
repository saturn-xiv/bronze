use std::fmt;
use std::result::Result as StdResult;

use actix_web::{dev::HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    StdIo(std::io::Error),
    StdNetAddrParse(std::net::AddrParseError),
    StdNumParseInt(std::num::ParseIntError),
    StdStrUtf8(std::str::Utf8Error),
    StdStringFromUtf8(std::string::FromUtf8Error),

    Askama(askama::Error),
    ActixWebBlockingSerdeJson(actix_web::error::BlockingError<serde_json::Error>),
    MimeFromStr(mime::FromStrError),
    SerdeJson(serde_json::Error),
    Validator(validator::ValidationErrors),

    Http(StatusCode, Option<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StdIo(v) => v.fmt(f),
            Self::StdNetAddrParse(v) => v.fmt(f),
            Self::StdNumParseInt(v) => v.fmt(f),
            Self::StdStrUtf8(v) => v.fmt(f),
            Self::StdStringFromUtf8(v) => v.fmt(f),

            Self::Askama(v) => v.fmt(f),
            Self::ActixWebBlockingSerdeJson(v) => v.fmt(f),
            Self::MimeFromStr(v) => v.fmt(f),
            Self::SerdeJson(v) => v.fmt(f),
            Self::Validator(v) => v.fmt(f),

            Self::Http(v, r) => match r {
                Some(r) => r.fmt(f),
                None => v.fmt(f),
            },
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::StdStringFromUtf8(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::StdIo(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::StdNumParseInt(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::StdStrUtf8(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::StdNetAddrParse(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Self {
        Self::Askama(err)
    }
}

impl From<actix_web::error::BlockingError<serde_json::Error>> for Error {
    fn from(err: actix_web::error::BlockingError<serde_json::Error>) -> Self {
        Self::ActixWebBlockingSerdeJson(err)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::Validator(err)
    }
}

impl From<mime::FromStrError> for Error {
    fn from(err: mime::FromStrError) -> Self {
        Self::MimeFromStr(err)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .content_type(mime::TEXT_PLAIN_UTF_8.to_string())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Http(it, _) => it,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
