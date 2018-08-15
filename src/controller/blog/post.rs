use template::BlogTemplate;

#[derive(Serialize, Deserialize)]
pub struct RawBlogPost {
    year: u16,
    month: u8,
    day: u8,
    title: String,
}

impl Into<BlogTemplate> for RawBlogPost {
    fn into(self) -> BlogTemplate {
        BlogTemplate::new(
            self.title,
            self.year,
            self.month,
            self.day,
        )
    }
}
