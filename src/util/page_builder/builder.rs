use std::path::PathBuf;
use std::result::Result::{Err as StdErr, Ok as StdOk};
use std::sync::{Arc, MutexGuard};

use actix_web::HttpResponse;
use handlebars::Handlebars;

use cache::{Cache, transform};
use cache::CacheResult::*;
use controller::ControllerError;

use super::*;

//
// Struct definitions
//

pub struct PageBuilder<'a> {
    key: Option<String>,
    state: PageState,
    cached: bool,

    body: Arc<String>,
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
            key: None,
            state: PageState::Ok,
            cached: false,
            body: Arc::new("".to_owned()),
            cache,
            hb,
        }
    }

    pub fn from_file(hb: &'a Handlebars, path: PathBuf) -> Self {
        let path = path.to_string_lossy().to_owned();

        let mut this = Self::new(hb);
        this.key = Some(path.clone().into());
        match this.cache.file_and_then(path, transform::strip_whitespace) {
            New(content) => this.body = content,
            Cached(content) => {
                this.body = content;
                this.cached = true;
            },
            Err(err) => {
                this.body = Arc::new(ControllerError::from(err).into());
                this.state = PageState::InternalServerError;
            }
        };
        this
    }

    pub fn not_found(hb: &'a Handlebars) -> Self {
        let mut this = Self::new(hb);
        this.state = PageState::NotFound;
        this
    }

//    pub fn cache_key<S>(mut self, key: S) -> Self
//    where
//        S: Into<String>,
//    {
//        let key = key.into();
//        println!("Searching for cache entry: '{}'", key);
//
//        // attempt to fetch the body from the cache if possible
//        match self.cache.entry(&key) {
//            New(_) => unreachable!("Constraint on Cache.entry violated!"),
//            Cached(body) => {
//                println!("Found in log: {}", key);
//                self.body = body;
//                self.cached = true;
//            },
//            Err(_) => {
//                // insert the current body into the cache, just to create the entry
//                self.body = self.cache.put(key.clone(), self.body.as_str().to_owned());
//            },
//        };
//
//        self.key = Some(key);
//
//        self
//    }

    pub fn render_template<T>(mut self, template: T) -> Self
    where
        T: PageTemplate {
        use util::template as file;

        if self.state != PageState::Ok || self.cached {
            return self;
        }

        let template_text = match self.cache.stripped_file(file(T::NAME)) {
            New(it)
            | Cached(it) => it,
            Err(err) => {
                self.body = Arc::new(ControllerError::from(err).into());
                self.state = PageState::InternalServerError;
                return self;
            },
        };

        let data = match template.data(self.body.clone(), &mut self.cache) {
            StdOk(it) => it,
            StdErr(err) => {
                self.body = Arc::new(err.into());
                self.state = PageState::InternalServerError;
                return self;
            },
        };

        match self.hb.render_template(&template_text, &data) {
            StdOk(text) => self.body = Arc::new(text),
            StdErr(err) => {
                self.body = Arc::new(ControllerError::from(err).into());
                self.state = PageState::InternalServerError;
            },
        };
        return self;
    }

    pub fn finish(mut self) -> HttpResponse {
        use super::PageState::*;

        // before we return the text, let's throw it back into the cache if we have a key
        // and it was successfully generated
        match (self.key, self.cached) {
            (Some(key), false) => self.cache.update(key, self.body.clone()),
            (_, _) => {},
        }

        let mut response = match self.state {
            Ok => HttpResponse::Ok(),
            NotFound => HttpResponse::NotFound(),
            InternalServerError => HttpResponse::InternalServerError(),
        };

        response.body(self.body)
    }
}

