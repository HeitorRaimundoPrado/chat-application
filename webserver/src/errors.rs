use crate::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum Error {
    InternalServerError,
    NotFoundError,
}

pub trait AsError {
    fn as_error(&self) -> Error;
}

pub fn adapt_error<T: AsError>(error: T) {
    error.as_error();
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InternalServerError => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
                    .into_response();
            }

            Error::NotFoundError => {
                return (StatusCode::NOT_FOUND, "Value not in database").into_response();
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotFoundError => write!(f, "Not found entry in db"),
            Error::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl AsError for diesel::result::Error {
    fn as_error(&self) -> Error {
        match self {
            diesel::result::Error::NotFound => Error::NotFoundError,
            _ => Error::InternalServerError,
        }
    }
}

impl AsError for deadpool_diesel::InteractError {
    fn as_error(&self) -> Error {
        match self {
            _ => Error::InternalServerError,
        }
    }
}
