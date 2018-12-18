use std::sync::{Arc, MutexGuard};

use cache::Cache;
use controller::Result;
use util::PageTemplate;

#[derive(Serialize)]
pub struct ServerTemplate {
    ip: Option<String>,
}

impl ServerTemplate {
    pub fn new(ip: Option<String>) -> Self {
        ServerTemplate {
            ip,
        }
    }
}

impl PageTemplate for ServerTemplate {
    const NAME: &'static str = "home.html";
    type TemplateData = Self;

    fn data(
        self,
        _body: Arc<String>,
        _cache: &mut MutexGuard<Cache>
    ) -> Result<Self::TemplateData> {
        Ok(self)
    }
}
