use std::error::Error;
use std::fmt;
use std::io;
use std::result;

use actix_web::HttpResponse;

use handlebars::TemplateRenderError;

pub type Result<T> = result::Result<T, ControllerError>;

#[derive(Debug)]
pub enum ControllerError {
    IoError(io::Error),
    TemplateRenderError(TemplateRenderError),
    String(String),
}

impl Into<HttpResponse> for ControllerError {
    fn into(self) -> HttpResponse {
        let msg = format!("{}", self);
        eprintln!("{}", msg);

        HttpResponse::InternalServerError()
            .body(msg)
    }
}

impl From<io::Error> for ControllerError {
    fn from(err: io::Error) -> Self {
        ControllerError::IoError(err)
    }
}

impl From<TemplateRenderError> for ControllerError {
    fn from(err: TemplateRenderError) -> Self {
        ControllerError::TemplateRenderError(err)
    }
}

impl From<String> for ControllerError {
    fn from(err: String) -> Self {
        ControllerError::String(err)
    }
}

impl<'a> From<&'a str> for ControllerError {
    fn from(err: &'a str) -> Self {
        ControllerError::String(err.to_owned())
    }
}

impl fmt::Display for ControllerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ControllerError::*;

        match self {
            IoError(err) =>
                write!(f, "IoError: {}", err),

            TemplateRenderError(err) =>
                write!(f, "TemplateFormatError: {}", err),

            String(msg) =>
                write!(f, "{}", msg),
        }
    }
}

impl Error for ControllerError {

}

