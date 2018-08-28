use std::error::Error;
use std::fmt;
use std::io;
use std::result;

use handlebars::{RenderError, TemplateError, TemplateRenderError};

pub type Result<T> = result::Result<T, ControllerError>;

#[derive(Debug)]
pub enum ControllerError {
    IoError(io::Error),
    TemplateRenderError(RenderError),
    TemplateFormatError(TemplateError),
    String(String),
}

impl Into<String> for ControllerError {
    fn into(self) -> String {
        format!("Sorry, a problem occurred!\n{}", self)
    }
}

impl From<io::Error> for ControllerError {
    fn from(err: io::Error) -> Self {
        ControllerError::IoError(err)
    }
}

impl From<TemplateRenderError> for ControllerError {
    fn from(err: TemplateRenderError) -> Self {
        use handlebars::TemplateRenderError::*;

        match err {
            IOError(err, _) => ControllerError::IoError(err),
            RenderError(err) => ControllerError::TemplateRenderError(err),
            TemplateError(err) => ControllerError::TemplateFormatError(err),
        }
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
                write!(f, "{}", err),

            TemplateRenderError(err) =>
                write!(f, "{}", err),

            TemplateFormatError(err) => {
                writeln!(f, "Template format error!")?;

                if let Some(ref name) = err.template_name {
                    writeln!(f, "Template name: {}", name)?;
                }

                match (err.line_no, err.column_no) {
                    (Some(ref line), Some(ref col)) => {
                        writeln!(f, "At line {}:{}", line, col)?;
                    },

                    (_, _) => {},
                };

                writeln!(f, "{}", err.reason)
            },

            String(msg) =>
                write!(f, "{}", msg),
        }
    }
}

impl Error for ControllerError {

}

