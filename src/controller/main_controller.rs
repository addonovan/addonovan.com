use std::sync::{Arc, MutexGuard};

use actix_web::{HttpResponse, HttpRequest};

use cache::{CACHE, Cache, transform};
use controller::{Controller, ControllerError, Result};
use decorator::{self, Decorator};
use handlebars::Handlebars;

pub struct MainController {
    resolver: decorator::FileResolver,
    hb: Handlebars,
}

#[derive(Serialize)]
struct PageInfo {
    style: Arc<String>,
    content: Arc<String>,
}

const CONTENT_DIR: &'static str = "res/content";
const TEMPLATE_DIR: &'static str = "res/template";

fn content<S>(item: S) -> String
    where S: AsRef<str> {
    format!("{}/{}", CONTENT_DIR, item.as_ref())
}

fn template<S>(item: S) -> String
    where S: AsRef<str> {
    format!("{}/{}", TEMPLATE_DIR, item.as_ref())
}

impl MainController {

    pub fn new() -> Self {
        MainController {
            resolver: decorator::FileResolver::new(CONTENT_DIR, false),
            hb: Handlebars::new(),
        }
    }

    fn render_content<'a>(
        &self,
        cache: &mut MutexGuard<'a, Cache>,
        body: Arc<String>
    ) -> Result<String> {

        let style = cache.file_and_then(
            content("style.css"),
            transform::strip_whitespace
        )?;

        let template = cache.file_and_then(
            template("format.html"),
            transform::strip_whitespace
        )?;

        let page = PageInfo {
            content: body,
            style
        };

        self.hb.render_template(&template, &page)
            .map_err(|_| "Failed to render template".into())
    }

    pub fn cache_overview(&self, _req: &HttpRequest) -> HttpResponse {
        let mut cache = CACHE.lock().unwrap();

        let template = match cache.file_and_then(
            template("cache_overview.html"),
            transform::strip_whitespace
        ) {
            Ok(it) => it,
            Err(err) => return ControllerError::from(err).into(),
        };

        let data = cache.overview();

        let content = match self.hb.render_template(&template, &data) {
            Ok(it) => Arc::new(it),
            Err(err) => return ControllerError::from(err).into(),
        };

        self.render_content(&mut cache, content)
            .map(|body| HttpResponse::Ok().body(body))
            .unwrap_or_else(|err| err.into())
    }

}

impl Controller for MainController {

    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        let mut cache = CACHE.lock().unwrap();

        // try to locate the file based on the URL, if we can't manage it, then
        // respond with a 404 page
        let path = match req.match_info()
            .query("tail")
            .ok()
            .and_then(|path| self.resolver.apply(path))
            .ok_or_else(|| ControllerError::from("Failed to locate file")) {

            Ok(it) => it,

            Err(_) => return match cache.file(content("404.html")) {
                Ok(page) => HttpResponse::NotFound().body(page),
                Err(e) => ControllerError::from(e).into()
            },
        };

        // convert the path to a string
        let path = path.into_os_string();
        let path = path.to_string_lossy();

        // get the contents of the file from the cache, and also check to see if
        // we have a new entry or not
        let mut new = false;
        let result = cache.file_and_then(
            path.clone(),
            |input| {
                new = true;
                transform::strip_whitespace(input)
            }
        );

        let body = match (result, new) {

            (Err(e), _) => return ControllerError::from(e).into(),

            // we need to format the text now
            (Ok(content), true) => {
                self.render_content(&mut cache, content)
                    .map(Arc::new)
                    .map(transform::strip_whitespace)
                    .map(|text| {
                        // "transform" the original content by replacing it
                        // with the next text
                        let copy = text.clone();
                        cache.transform(path, |_| copy);
                        text
                    })
            },

            (Ok(content), false) => Ok(content)
        };

        body.map(|body| HttpResponse::Ok().body(body))
            .unwrap_or_else(|e| e.into())
    }

}
