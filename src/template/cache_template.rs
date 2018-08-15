use std::sync::{Arc, MutexGuard};

use cache::{Cache, CacheOverview};
use controller::Result;
use util::PageTemplate;

pub struct CacheOverviewTemplate;

impl CacheOverviewTemplate {
    pub fn new() -> Self { CacheOverviewTemplate }
}

impl PageTemplate for CacheOverviewTemplate {
    const NAME: &'static str = "cache_overview.html";
    type TemplateData = CacheOverview;

    fn data(
        self,
        _body: Arc<String>,
        cache: &mut MutexGuard<Cache>
    ) -> Result<Self::TemplateData> {
        Ok(cache.overview())
    }
}

