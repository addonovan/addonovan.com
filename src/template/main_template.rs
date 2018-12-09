use std::sync::{Arc, MutexGuard};
use std::time::Instant;

use cache::Cache;
use controller::Result;
use util::PageTemplate;
use util::{style, current_date, elapsed};

#[derive(Serialize)]
pub struct PageInfo {
    title: Option<String>,
    styles: Vec<String>,
    content: Arc<String>,
    year: i32,
    month: u32,
    day: u32,
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
        _cache: &mut MutexGuard<Cache>
    ) -> Result<Self::TemplateData> {
        let (year, month, day) = current_date();

        Ok(PageInfo {
            title: self.title,
            styles: self.styles,
            content,
            year,
            month,
            day,
            elapsed_time: elapsed(self.start),
        })
    }
}
