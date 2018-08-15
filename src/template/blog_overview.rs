use std::sync::{Arc, MutexGuard};

use cache::Cache;
use controller::Result;
use util::PageTemplate;

use super::BlogTemplate;

#[derive(Serialize)]
pub struct BlogOverviewTemplate {
    posts: Arc<Vec<BlogTemplate>>,
}

impl BlogOverviewTemplate {
    pub fn new(posts: Arc<Vec<BlogTemplate>>) -> Self {
        BlogOverviewTemplate {
            posts,
        }
    }
}

impl PageTemplate for BlogOverviewTemplate {
    const NAME: &'static str = "blog_overview.html";
    type TemplateData = Self;

    fn data(
        self,
        _body: Arc<String>,
        _cache: &mut MutexGuard<Cache>
    ) -> Result<Self::TemplateData> {
        Ok(self)
    }
}
