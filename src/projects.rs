use mwf;
use mwf::{RequestHandler, View, RouteMap, decorator};

use config::CONFIG;
use decs::{Processor, Substitute};

pub struct ProjectController
{
    subst: Substitute,
    md: decorator::Markdown,
    format: decorator::Surround,
    processor: Processor,
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
            subst: Substitute,
            md: decorator::Markdown,
            format,
            processor: Processor::new(),
        }
    }
}

impl RequestHandler for ProjectController
{
    fn handle(&self, route_map: RouteMap) -> mwf::Result<View>
    {
        use std::path::PathBuf;

        // default the file name to "index" if it's empty
        let file = route_map.get(":page?")
            .map(|x| if x.is_empty() { "index" } else { x })
            .map(|x| format!("res/projects/{}", x))
            .expect("Dude, how did this happen");

        // if the file has an extension, then we'll serve it directly
        if let Some(_) = PathBuf::from(&file).extension() {
            return View::file(file);
        }

        let file = format!("{}.md", file);

        let content = View::file(file)?
            .apply(&self.subst)
            .apply(&self.md);

        // if we're in debug mode, then we'll never cache the contents of the
        // format file, so we'll always re-read the format.html file
        let content = match CONFIG.debug {
            true => {
                use std::fs::File;
                use std::io::Read;

                let mut file = File::open("res/projects/format.html")?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let dec = decorator::Surround::from(contents);
                content.apply(&dec)
            },

            false => {
                content.apply(&self.format)
            }
        };

        Ok(content.apply(&self.subst).apply(&self.processor))
    }
}
