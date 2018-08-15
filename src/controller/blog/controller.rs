use std::str::FromStr;
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse};
use actix_web::dev::Params;
use handlebars::Handlebars;

use controller::Controller;
use template::{BlogOverviewTemplate, BlogTemplate, MainTemplate};
use util::PageBuilder;

pub struct BlogController {
    hb: Handlebars,
    posts: Arc<Vec<BlogTemplate>>,
}

fn match_req<T>(param: &'static str, match_info: &Params) -> Option<T>
where
    T: FromStr,
{
    let text: String = match_info.query(param).ok()?;
    text.parse::<T>().ok()
}

fn post_date(req: &HttpRequest) -> Option<(u16, u8, u8)> {
    let match_info = req.match_info();

    let year: u16 = match_req("year", match_info)?;
    let month: u8 = match_req("month", match_info)?;
    let day: u8 = match_req("day", match_info)?;

    Some((year, month, day))
}

fn make_post_links(posts: &mut Vec<BlogTemplate>) {
    // the list is REVERSE chronological, so the first element is the newest
    let mut next: Option<&mut BlogTemplate> = None;
    for post in posts {
        if let Some(next) = next {
            next.prev(post);
            post.next(next);
        }

        next = Some(post);
    }
}

fn discover_blogs() -> Vec<BlogTemplate> {
    use serde_json;
    use cache::CACHE;
    use constants::BLOG_DIR;
    use super::RawBlogPost;

    let mut cache = CACHE.lock().expect("Failed to lock file cache!");
    let file = cache.stripped_file(format!("{}/posts.json", BLOG_DIR))
        .expect("Couldn't load blog post overview!");

    let mut posts: Vec<BlogTemplate> = serde_json::from_str(&file)
        .map(|list: Vec<RawBlogPost>| list.into_iter()
            .map(|post| post.into())
            .collect()
        )
        .expect("Failed to load posts.json!");

    make_post_links(&mut posts);
    posts
}

impl BlogController {
    pub fn new() -> Self {
        // TODO make this cached somehow
        let posts = Arc::new(discover_blogs());

        BlogController {
            hb: Handlebars::new(),
            posts,
        }
    }

    fn post<'a>(&'a self, req: &HttpRequest) -> Option<&'a BlogTemplate> {
        let (year, month, day) = post_date(req)?;
        self.posts.iter().find(|it| {
            it.year == year && it.month == month && it.day == day
        })
    }

    fn response(
        &self,
        mut main: MainTemplate,
        post: BlogTemplate
    ) -> HttpResponse {
        main.style("blog.css");
        main.title(post.title());

        PageBuilder::new(&self.hb)
            .render_template(post)
            .render_template(main)
            .finish()
    }

    pub fn overview(&self, _req: &HttpRequest) -> HttpResponse {
        let main = MainTemplate::new();
        let overview = BlogOverviewTemplate::new(self.posts.clone());

        PageBuilder::new(&self.hb)
            .render_template(overview)
            .render_template(main)
            .finish()
    }
}

impl Controller for BlogController {
    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        let main = MainTemplate::new();
        match self.post(req) {
            Some(post) => self.response(main, post.clone()),
            None => PageBuilder::not_found(&self.hb).finish(),
        }
    }
}
