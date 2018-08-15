use std::path::PathBuf;
use std::sync::{Arc, MutexGuard};

use actix_web::HttpResponse;
use handlebars::Handlebars;

use cache::Cache;
use controller::ControllerError;
use cache::transform;

use super::*;

//
// Struct definitions
//

pub struct PageBuilder<'a> {
    state: PageState,
    cache: MutexGuard<'a, Cache>,
    hb: &'a Handlebars,
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

    pub fn render_template<T>(mut self, template: T) -> Self
    where T: PageTemplate {
        use util::template as file;

        let body = match self.state {
            PageState::Ok(ref content) => content.clone(),
            _ => return self,
        };

        let template_text = match self.cache.stripped_file(file(T::NAME)) {
            Ok(it) => it,
            Err(err) => {
                self.state = PageState::InternalServerError(err.into());
                return self;
            },
        };

        let data = match template.data(body, &mut self.cache) {
            Ok(it) => it,
            Err(err) => {
                self.state = PageState::InternalServerError(err);
                return self;
            },
        };

        match self.hb.render_template(&template_text, &data) {
            Ok(text) => self.state = PageState::Ok(Arc::new(text)),
            Err(err) => self.state = PageState::InternalServerError(err.into()),
        };
        return self;
    }

    pub fn finish(self) -> HttpResponse {
        use super::PageState::*;

        match self.state {
            Ok(body) => HttpResponse::Ok().body(body),

            NotFound => HttpResponse::NotFound().body("Ugh"),

            InternalServerError(err) => {
                let message = format!("Sorry, a problem occurred\n{}", err);
                HttpResponse::InternalServerError()
                    .body(message)
            },
        }
    }
}

