use decorator::{self, Decorator};
use handlebars::Handlebars;
use controller::Controller;

use actix_web::{HttpResponse, HttpRequest};
use std::path::PathBuf;

use file_cache::CACHE;

use std::sync::Arc;
use file_cache::CacheOverviewEntry;

pub struct MainController {
    resolver: decorator::FileResolver,
    hb: Handlebars,
}

#[derive(Serialize)]
struct PageInfo {
    style: Arc<String>,
    content: Arc<String>,
}

#[derive(Serialize)]
struct CacheOverview {
    total_size: usize,
    entries: Vec<CacheOverviewEntry>,
}

impl MainController {

    pub fn new() -> Self {
        MainController {
            resolver: decorator::FileResolver::new("res/content", false),
            hb: Handlebars::new(),
        }
    }

    fn render_content(&self, content: Arc<String>)
        -> Result<String, String> {

        let style: Arc<String>;
        let template: Arc<String>;
        {
            let mut cache = CACHE.lock().unwrap();

            style = match cache.get("res/content/style.css") {
                Some(it) => it,
                None => return Err(
                    format!("Failed to retrieve `res/content/style.css`" )
                ),
            };

            template = match cache.get("res/template/format.html") {
                Some(it) => it,
                None => return Err(
                    format!("Failed to retrieve `res/template/format.html`")
                )
            };
        }

        let page = PageInfo {
            content,
            style
        };

        self.hb.render_template(&template, &page)
            .map_err(|_| "Failed to render template".into())
    }

    pub fn cache_overview(&self, _req: &HttpRequest) -> HttpResponse {
        let content: Arc<String>;
        {
            let mut cache = CACHE.lock().unwrap();

            let template = match cache.get("res/template/cache_overview.html") {
                Some(it) => it,
                None => return HttpResponse::InternalServerError()
                    .body("Could not find `res/template/cache_overview.html`"),
            };

            let data = cache.overview();
            let data = CacheOverview {
                total_size: data.iter().map(|x| x.size).sum(),
                entries: data,
            };

            content = match self.hb.render_template(&template, &data) {
                Ok(it) => Arc::new(it),
                Err(e) => return HttpResponse::InternalServerError()
                    .body(format!("Template format failure: {:?}", e))
            };
        }

        let content = match self.render_content(content) {
            Ok(it) => it,
            Err(e) => return HttpResponse::InternalServerError().body(e)
        };

        HttpResponse::Ok()
            .body(content)
    }

}

impl Controller for MainController {

    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        let content: Arc<String>;

        {
            let mut cache = CACHE.lock().unwrap();

            let content_option = req.match_info()
                .query("tail")
                .ok()
                .and_then(|path: PathBuf| self.resolver.apply(path))
                .and_then(|path: PathBuf| cache.get(path));

            content = match content_option {
                Some(it) => it,
                None => cache.get("res/content/404.html")
                    .expect("No 404 page is set up!")
            };
        }
        let content = match self.render_content(content) {
            Ok(it) => it,
            Err(e) => return HttpResponse::InternalServerError().body(e)
        };

        HttpResponse::Ok()
            .body(content)
    }

}
