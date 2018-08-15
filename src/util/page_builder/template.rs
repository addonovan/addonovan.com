use std::sync::{Arc, MutexGuard};

use serde::Serialize;

use cache::Cache;
use controller::Result;

pub trait PageTemplate {

    const NAME: &'static str;

    type TemplateData: Serialize;

    fn data(
        self,
        body: Arc<String>,
        cache: &mut MutexGuard<Cache>
    ) -> Result<Self::TemplateData>;

}

