use std::path::PathBuf;

use actix_web::{HttpResponse, HttpRequest};
use handlebars::Handlebars;

use controller::{Controller, ControllerError, Result};
use decorator::{Decorator, FileResolver};

use super::builder::PageBuilder;

pub struct MainController {
    resolver: FileResolver,
    hb: Handlebars,
}

impl MainController {

    pub fn new() -> Self {
        use super::CONTENT_DIR;
        MainController {
            resolver: FileResolver::new(CONTENT_DIR, false),
            hb: Handlebars::new(),
        }
    }

    fn match_tail(&self, req: &HttpRequest) -> Result<PathBuf> {
        req.match_info()
            .query("tail")
            .ok()
            .and_then(|path| self.resolver.apply(path))
            .ok_or_else(|| ControllerError::from("Failed to find file"))
    }

    pub fn cache_overview(&self, _req: &HttpRequest) -> HttpResponse {
        let data = {
            use cache::CACHE;
            let cache = CACHE.lock().expect("Failed to lock file cache");
            cache.overview()
        };

        PageBuilder::from_template(&self.hb, "cache_overview.html", &data)
            .render_template("format.html")
            .finish()
    }

}

impl Controller for MainController {

    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        let builder = match self.match_tail(req) {
            Err(_) => PageBuilder::not_found(&self.hb),
            Ok(path) => PageBuilder::from_file(&self.hb, path),
        };

        builder.render_template("format.html")
            .finish()
    }

}
