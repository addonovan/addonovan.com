use serde::Serialize;

use decorator::Decorator;
use handlebars::Handlebars;

pub struct Substitution
{
    hb: Handlebars,
}

impl From<Handlebars> for Substitution
{
    fn from(hb: Handlebars) -> Self
    {
        Substitution {
            hb,
        }
    }
}

impl<I> Decorator<(String, I), Option<String>> for Substitution
    where I: Serialize
{
    fn apply(&self, input: (String, I)) -> Option<String>
    {
        let (template, data) = input;
        self.hb.render_template(&template, &data)
//            .and_then(|text: String| {
//                self.hb.render_template(&text, &())
//            })
            .map_err(|err| {
                println!("Failed to apply template: {:?}", err);
                ()
            })
            .ok()
    }
}
