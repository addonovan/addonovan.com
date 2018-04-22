use chrono::{Local, Datelike};
use mwf::{View, Decorator};
use regex::{Regex, Captures};

fn replace_text(caps: &Captures) -> String
{
    let input = &caps[1];
    match input {
        "Dirs.Root" => "".into(),
        "Dirs.Projects" => "/projects".into(),

        "Date" => {
            let local = Local::now();
            format!("{}.{}.{}", local.year(), local.month(), local.day())
        },
        "Date.Year"  => format!("{}", Local::now().year()),
        "Date.Month" => format!("{}", Local::now().month()),
        "Date.Day"   => format!("{}", Local::now().day()),

        _ => format!("{{ERR: Could not find string:{}}}", input),
    }
}

pub struct Substitute;
impl Decorator for Substitute
{
    fn decorate(&self, mut view: View) -> View
    {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"\{\{([A-Za-z0-9\.]+)}}"
            ).unwrap();
        }

        view.content = REGEX.replace_all(&view.content, replace_text)
            .to_string();

        view
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use mwf::View;

    #[test]
    fn replacement_replaces_dirs()
    {
        let dec = Substitute;
        let view = View::raw("{{Dirs.Root}}").apply(&dec);
        assert_eq!(view.content, "");
    }
}
