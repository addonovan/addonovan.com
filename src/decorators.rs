use mwf::{ViewResult, View, ViewDecorator};
use std::path::PathBuf;
use std::fs::File;
use std::io::{Error, Read};

/// A simple decorator which will surround the content of the view with the
/// specified text.
pub struct SurroundDecorator
{
    /// The stuff that goes before the view's content.
    pre: String,

    /// The stuff that goes after the view's content.
    post: String,
}

#[derive(Debug)]
pub enum DecoratorError
{
    BadInput,
    IoError(Error)
}

impl SurroundDecorator
{
    /// Creates a new decorator which will output a view whose content matches:
    /// `before` `view content` `after`.
    pub fn new<T: Into<String>, U: Into<String>>(before: T, after: U) -> Self
    {
        SurroundDecorator {
            pre: before.into(),
            post: after.into(),
        }
    }

    /// Reads the contents of a file and splits it on the delimiter
    /// `{{content}}` into two parts and if
    pub fn file<T: Into<PathBuf>>(path: T) -> Result<Self, DecoratorError>
    {
        // read the file contents
        let mut file = File::open(path.into())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // now we should split on the {{content}} delimiter
        let mut split: Vec<String> = contents.split("{{content}}")
            .map(String::from)
            .collect();

        // we have to have at least two parts
        if split.len() != 2 {
            return Err(DecoratorError::BadInput);
        }

        Ok(SurroundDecorator::new(
            split.remove(0),
            split.remove(0)
        ))
    }
}

impl ViewDecorator for SurroundDecorator
{
    fn decorate(&self, view: View) -> ViewResult
    {
        let (content, _) = view.into();

        let content = format!("{}{}{}", self.pre, content, self.post);

        View::from(content).and_then(|mut it| {
            it.mime("text/html".parse().unwrap());
            Ok(it)
        })
    }
}

impl From<Error> for DecoratorError
{
    fn from(err: Error) -> Self
    {
        DecoratorError::IoError(err)
    }
}
