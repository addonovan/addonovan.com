use std::path::PathBuf;

use actix_web::{HttpResponse, HttpRequest};
use handlebars::Handlebars;

use controller::{Controller, ControllerError, Result};
use decorator::{Decorator, FileResolver};
use template::{MainTemplate, CacheOverviewTemplate};
use util::PageBuilder;

pub struct MainController {
    resolver: FileResolver,
    hb: Handlebars,
}

impl MainController {

    pub fn new() -> Self {
        use constants::MAIN_DIR;
        MainController {
            resolver: FileResolver::new(MAIN_DIR, false),
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
        let main = MainTemplate::new();
        let overview = CacheOverviewTemplate::new();

        PageBuilder::new(&self.hb)
            .render_template(overview)
            .render_template(main)
            .finish()
    }

}

impl Controller for MainController {

    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        let template = MainTemplate::new();

        let builder = match self.match_tail(req) {
            Err(_) => PageBuilder::not_found(&self.hb),
            Ok(path) => PageBuilder::from_file(&self.hb, path),
        };

        builder.render_template(template)
            .finish()
    }

}
