use std::path::PathBuf;
use std::sync::{Arc, MutexGuard};

use actix_web::HttpResponse;
use handlebars::Handlebars;
use serde::Serialize;

use cache::Cache;
use controller::{ControllerError, Result};
use cache::transform;

//
// File-helpers
//

fn content<S>(item: S) -> String
    where S: AsRef<str> {
    use super::CONTENT_DIR;
    format!("{}/{}", CONTENT_DIR, item.as_ref())
}

fn template<S>(item: S) -> String
    where S: AsRef<str> {
    use super::TEMPLATE_DIR;
    format!("{}/{}", TEMPLATE_DIR, item.as_ref())
}

//
// Struct definitions
//

pub struct PageBuilder<'a> {
    state: PageState,
    cache: MutexGuard<'a, Cache>,
    hb: &'a Handlebars,
}

enum PageState {
    Ok(Arc<String>),
    NotFound,
    InternalServerError(ControllerError),
}

#[derive(Serialize)]
struct PageInfo {
    style: Arc<String>,
    content: Arc<String>,
}

//
// Implementation
//

impl<'a> PageBuilder<'a> {

    pub fn new(hb: &'a Handlebars) -> Self {
        use cache::CACHE;
        let cache = CACHE.lock().expect("Couldn't lock file cache");

        PageBuilder {
            state: PageState::Ok(Arc::new("".to_owned())),
            cache,
            hb,
        }
    }

    pub fn from_template<S, Sz>(hb: &'a Handlebars, name: S, data: Sz) -> Self
        where S: AsRef<str>, Sz: Serialize {

        let mut this = Self::new(hb);
        let template = match this.file(template(name)) {
            Ok(content) => content,
            Err(err) => {
                this.state = PageState::InternalServerError(err);
                return this;
            }
        };

        match hb.render_template(&template, &data) {
            Ok(content) => this.state = PageState::Ok(Arc::new(content)),
            Err(err) => this.state = PageState::InternalServerError(err.into()),
        };

        this
    }

    pub fn from_file(hb: &'a Handlebars, path: PathBuf) -> Self {
        let path = path.to_string_lossy().to_owned();

        let mut this = Self::new(hb);
        match this.cache.file_and_then(path, transform::strip_whitespace) {
            Ok(content) => this.state = PageState::Ok(content),
            Err(err) => {
                let err = ControllerError::from(err);
                this.state = PageState::InternalServerError(err);
            }
        };
        this
    }

    pub fn not_found(hb: &'a Handlebars) -> Self {
        let mut this = Self::new(hb);
        this.state = PageState::NotFound;
        this
    }

    fn file(&mut self, path: String) -> Result<Arc<String>> {
        self.cache.file_and_then(path, transform::strip_whitespace)
            .map_err(ControllerError::from)
    }

    fn invoke_template(
        &mut self,
        template: &str,
        page: &PageInfo
    ) -> Result<Arc<String>> {
        self.hb.render_template(template, page)
            .map_err(ControllerError::from)
            .map(|text| Arc::new(text))
    }

    pub fn render_template<S: AsRef<str>>(mut self, template_name: S) -> Self {
        // this only applies to successful pages
        let body = match self.state {
            PageState::Ok(ref content) => content.clone(),
            _ => return self,
        };

        let style = self.file(content("style.css"));
        let template = self.file(template(template_name));

        let (style, template) = match (style, template) {
            (Err(err), _)
            | (_, Err(err)) => {
                let err = ControllerError::from(err);
                self.state = PageState::InternalServerError(err);
                return self;
            },

            (Ok(style), Ok(template)) => (style, template),
        };

        let page = PageInfo {
            style,
            content: body,
        };

        match self.invoke_template(&template, &page) {
            Ok(text) => self.state = PageState::Ok(text),
            Err(err) => self.state = PageState::InternalServerError(err),
        };
        return self;
    }

    pub fn finish(self) -> HttpResponse {
        use self::PageState::*;

        match self.state {
            Ok(body) => HttpResponse::Ok().body(body),

            NotFound => HttpResponse::NotFound().body("Ugh"),

            InternalServerError(err) => {
                let message = format!("Sorry, a problem occurred\n{:?}", err);
                HttpResponse::InternalServerError()
                    .body(message)
            },
        }
    }
}

