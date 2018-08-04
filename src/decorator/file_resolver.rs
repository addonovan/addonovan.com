use std::path::PathBuf;

use decorator::Decorator;

const EXTENSION_PRIORITY: [&'static str; 2] = [
    "html",
    "md",
];

pub struct FileResolver
{
    path: &'static str,
    list_dirs: bool,
}

impl FileResolver
{
    pub fn new(path: &'static str, list_dirs: bool) -> Self
    {
        FileResolver {
            path,
            list_dirs,
        }
    }
}

impl Decorator<PathBuf, Option<PathBuf>> for FileResolver
{
    fn apply(&self, input: PathBuf) -> Option<PathBuf>
    {
        let mut output = PathBuf::from(self.path);
        output.push(&input);

        // if we're configured to not list directories and we've found a dir,
        // then we're going to search instead for the index file within the
        // directory
        if output.is_dir() {
            if self.list_dirs {
                return Some(output);
            } else {
                output.push("index");
            }
        }

        // if the file exists, then we'll match that immediately
        if output.exists() {
            return Some(output);
        }

        // otherwise, we'll go through our prioritized extensions list and check
        // for each of those files
        for ext in &EXTENSION_PRIORITY {
            output.set_extension(ext);

            if output.exists() {
                return Some(output);
            }
        }

        // we didn't find the file, so we'll give up
        println!("Failed to locate resource: {:?}", input);
        None
    }
}
