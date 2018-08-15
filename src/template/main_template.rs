use std::sync::{Arc, MutexGuard};
use std::time::Instant;

use cache::Cache;
use controller::Result;
use util::PageTemplate;
use util::{style, current_year, elapsed};

#[derive(Serialize)]
pub struct PageInfo {
    title: Option<String>,
    styles: Vec<Arc<String>>,
    content: Arc<String>,
    year: i32,
    elapsed_time: u64,
}

pub struct MainTemplate {
    start: Instant,
    styles: Vec<String>,
    title: Option<String>,
}

impl MainTemplate {

    pub fn new() -> Self {
        MainTemplate {
            title: None,
            start: Instant::now(),
            styles: vec![style("main.css")],
        }
    }

    pub fn style<S>(&mut self, name: S)
    where
        S: AsRef<str>
    {
        self.styles.push(style(name));
    }

    pub fn title<S>(&mut self, name: S)
    where
        S: Into<String>
    {
        self.title = Some(name.into());
    }
}

impl PageTemplate for MainTemplate {
    const NAME: &'static str = "main.html";
    type TemplateData = PageInfo;

    fn data(
        self,
        content: Arc<String>,
        cache: &mut MutexGuard<Cache>
    ) -> Result<Self::TemplateData> {
        use controller::ControllerError;

        let styles = self.styles.into_iter()
            .map(|it| cache.stripped_file(it).map_err(ControllerError::from))
            .collect::<Result<Vec<Arc<String>>>>()?;

        Ok(PageInfo {
            title: self.title,
            styles,
            content,
            year: current_year(),
            elapsed_time: elapsed(self.start),
        })
    }
}
