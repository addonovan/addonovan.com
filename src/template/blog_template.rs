use std::sync::{Arc, MutexGuard};
use std::path::PathBuf;

use cache::Cache;
use controller::Result;
use util::PageTemplate;

#[derive(Serialize)]
pub struct BlogPost {
    title: Arc<String>,
    body: Arc<String>,
    year: u16,
    month: u8,
    day: u8,

    prev: Option<PostLink>,
    next: Option<PostLink>,
}

#[derive(Clone, Serialize)]
struct PostLink {
    link: Arc<String>,
    title: Arc<String>,
}

#[derive(Clone, Serialize)]
pub struct BlogTemplate {
    title: Arc<String>,
    path: Arc<String>,
    link: Arc<String>,

    pub year: u16,
    pub month: u8,
    pub day: u8,

    prev: Option<PostLink>,
    next: Option<PostLink>,
}

impl BlogTemplate {

    fn path_for(year: u16, month: u8, day: u8) -> String {
        use constants::BLOG_DIR;

        format!(
            "{}/{:04}/{:02}/{:02}.html",
            BLOG_DIR,
            year,
            month,
            day
        )
    }

    fn link_for(year: u16, month: u8, day: u8) -> String {
        format!(
            "/blog/{:04}/{:02}/{:02}",
            year,
            month,
            day
        )
    }

    pub fn new(
        title: String,
        year: u16,
        month: u8,
        day: u8
    ) -> Self {
        BlogTemplate {
            title: Arc::new(title),
            path: Arc::new(Self::path_for(year, month, day)),
            link: Arc::new(Self::link_for(year, month, day)),
            year,
            month,
            day,
            prev: None,
            next: None,
        }
    }

    fn as_link(&self) -> PostLink {
        PostLink {
            title: self.title.clone(),
            link: self.link.clone(),
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.as_str().into()
    }

    pub fn prev(&mut self, prev: &Self) {
        self.prev = Some(prev.as_link());
    }

    pub fn next(&mut self, next: &Self) {
        self.next = Some(next.as_link());
    }

    pub fn title(&self) -> String {
        self.title.as_str().to_owned()
    }

}

impl PageTemplate for BlogTemplate {
    const NAME: &'static str = "blogpost.html";
    type TemplateData = BlogPost;

    fn data(
        self,
        body: Arc<String>,
        _cache: &mut MutexGuard<Cache>
    ) -> Result<Self::TemplateData> {

        Ok(BlogPost {
            title: self.title,
            body,
            year: self.year,
            month: self.month,
            day: self.day,
            prev: self.prev,
            next: self.next,
        })
    }
}
