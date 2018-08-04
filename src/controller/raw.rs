use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use actix_web::{HttpRequest, HttpResponse, Responder};

use controller::Controller;
use decorator;
use decorator::Decorator;

const DIRECTORY: &'static str = "res/raw";

pub struct Raw
{
    resolver: decorator::FileResolver,
    reader: decorator::NamedFileReader,
}

impl Raw
{
    pub fn new() -> Self
    {
        Raw {
            resolver: decorator::FileResolver::new(DIRECTORY, true),
            reader: decorator::NamedFileReader,
        }
    }
}

impl Controller for Raw
{
    fn handle(&self, req: &HttpRequest) -> HttpResponse
    {
        req.match_info()
            .query("tail")
            .ok()
            .and_then(|path: PathBuf| self.resolver.apply(path))
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "oh no!"))
            .and_then(|path: PathBuf| self.reader.apply(path))
            .and_then(|file| file.respond_to(&req))
            .into()
    }
}

