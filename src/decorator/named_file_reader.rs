use std::io::Result;
use std::path::PathBuf;

use actix_web::fs::NamedFile;

use decorator::Decorator;

pub struct NamedFileReader;

impl Decorator<PathBuf, Result<NamedFile>> for NamedFileReader
{
    fn apply(&self, input: PathBuf) -> Result<NamedFile>
    {
        NamedFile::open(input)
    }
}

