use mwf;
use mwf::{RequestHandler, View, RouteMap, decorator};

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
        let page = route_map.get(":page?");

        let file_path = match page {

            // default to the index page if there was none
            None => "res/projects/index.md".into(),

            Some(route) => {
                // replace the .html from the URL with a .md
                let route = route.trim_right_matches(".html")
                    .trim_right_matches(".htm");

                format!("res/projects/{}.md", route)
            }
        };

        View::file(file_path)
            .map(|view| view.apply(&self.md).apply(&self.format))
    }
}
