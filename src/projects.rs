use mwf;
use mwf::{RequestHandler, View, RouteMap, decorator};

use config::CONFIG;

pub struct ProjectController
{
    md: decorator::Markdown,
    format: decorator::Surround,
}

impl ProjectController
{
    /// Creates a new project controller
    pub fn new() -> Self
    {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open("res/projects/format.html")
            .expect("Failed to open projects format file");

        let mut format = String::new();
        file.read_to_string(&mut format)
            .expect("Failed to read projects format file");

        let format = decorator::Surround::from(format);

        ProjectController {
            md: decorator::Markdown,
            format,
        }
    }
}

impl RequestHandler for ProjectController
{
    fn handle(&self, route_map: RouteMap) -> mwf::Result<View>
    {
        let page = route_map.get(":page?")
            .expect("Dude, how did this happen");

        let file_path: String;
        if page.is_empty() {
            file_path = "res/projects/index.md".into();
        }
        else {
            let page = page.trim_right_matches(".html")
                .trim_right_matches(".htm");

            file_path = format!("res/projects/{}.md", page);
        }

        let content = View::file(file_path).map(|view| view.apply(&self.md));

        // if we're in debug mode, then we'll never cache the contents of the
        // format file, so we'll always re-read the format.html file
        if CONFIG.debug {
            use std::fs::File;
            use std::io::Read;

            let mut file = File::open("res/projects/format.html")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let dec = decorator::Surround::from(contents);
            return content.map(|view| view.apply(&dec));
        }

        content.map(|view| view.apply(&self.format))
    }
}
