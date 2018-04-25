use chrono::{Local, Datelike};
use mwf::{View, Decorator};
use regex::{Regex, Captures};

//
// Standard Substitutions
//

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

        // oh lord that's a lot of braces
        // it's literally just {{input}} though
        _ => format!("{{{{{}}}}}", input),
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

//
// Preprocessing Engine
//

use std::collections::HashMap;

use serde::Serialize;
use serde_json;
use serde_json::Value;
use serde_json::Error as SerdeError;

pub struct Processor
{
    pub args: HashMap<String, Value>,
}

impl Processor
{
    pub fn new() -> Self
    {
        Processor {
            args: HashMap::new(),
        }
    }

    pub fn clear(&mut self)
    {
        self.args.clear();
    }

    pub fn put<S, T>(&mut self, name: S, item: T) -> Result<(), SerdeError>
        where S: Into<String>,
              T: Serialize,
    {
        let name = name.into();
        let value = serde_json::to_value(item)?;
        self.args.insert(name, value);
        Ok(())
    }
}

fn process_control(captures: &Captures, map: &HashMap<String, Value>) -> String
{
    if captures.name("IF").is_some() {
        let condition = captures.name("CONDITION").unwrap();
        let text = captures.name("T").unwrap().as_str();

        match map.get(condition.as_str()) {
            None => "",
            Some(_) => text,
        }.into()
    }
    else if captures.name("FOREACH").is_some() {
        let iter = captures.name("ITER").unwrap().as_str();
        let list = captures.name("LIST").unwrap().as_str();
        let text = captures.name("T").unwrap().as_str().to_string();

        match map.get(list) {
            None => "".into(),
            Some(&Value::Array(ref list)) => {
                let mut out = String::new();

                for i in 0..list.len() {
                    let mut map: HashMap<String, Value> = HashMap::new();
                    map.insert(iter.into(), list.get(i).unwrap().clone());
                    out += &process_text(&text, &map);
                }

                out
            },
            _ => {
                format!("{} is not a list", list)
            }
        }
    }
    else {
        "Invalid control sequence".into()
    }
}

fn process_variable<S>(name: S, map: &HashMap<String, Value>) -> String
    where S: Into<String>
{
    let full_name = name.into();
    let mut names: Vec<&str> = full_name.split(".").collect();

    let name = names.remove(0);

    let text: String = match map.get(name) {
        None => format!("RefError: `{}`", name),
        Some(value) => {
            let mut value = Some(value);

            while !names.is_empty() {
                value = value.and_then(|it| it.get(names.remove(0)));
            }

            value.map(|val| {
                if let &Value::String(ref x) = val {
                    x.clone()
                }
                else {
                    serde_json::to_string(val).unwrap()
                }
            }).unwrap_or_else(|| format!("RefError: `{}`", full_name))
        }
    };
    text
}

fn process_text(text: &String, map: &HashMap<String, Value>) -> String
{
    lazy_static! {
        static ref CONTROL_REGEX: Regex = Regex::new(
            "(?s)\
                [^\\\\]?@(?:(?P<IF>if) (?P<CONDITION>\\w+)\
                |(?P<FOREACH>foreach) (?P<ITER>\\w+) in (?P<LIST>\\w+)) \
                \\{\\s+\
                    (?P<T>.*?)\\s+\
                \\}"
        ).unwrap();

        static ref REPLACE_REGEX: Regex = Regex::new(
            r"\$\{(?P<NAME>[\w\.]+)\}"
        ).unwrap();
    }

    let mut copy = text.clone();
    loop {
        let mut dirty = false;

        copy = CONTROL_REGEX.replace_all(&copy, |c: &Captures| {
            dirty = true;

            process_control(c, map)
        }).to_string();

        if dirty {
            continue;
        }

        copy = REPLACE_REGEX.replace_all(&copy, |c: &Captures| {
            dirty = true;

            let name = c.name("NAME").unwrap().as_str();
            process_variable(name, map)
        }).to_string();

        if dirty {
            continue;
        }

        break;
    }

    copy
}

impl Decorator for Processor
{
    fn decorate(&self, mut view: View) -> View
    {
        let new_text = process_text(&view.content, &self.args);
        view.content = new_text;
        view
    }
}

//
// Tests
//

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

    #[test]
    fn replacement_ignores_invalid_keys()
    {
        let dec = Substitute;
        let view = View::raw("{{hummina}}").apply(&dec);
        assert_eq!(view.content, "{{hummina}}");
    }

    #[derive(Serialize)]
    struct Person
    {
        name: String,
        age: u8,
    }

    #[test]
    fn processor_replaces_variables()
    {
        let mut dec = Processor::new();
        dec.put("person", "Austin Donovan").unwrap();
        let view = View::raw("${person}").apply(&dec);

        assert_eq!(view.content, "Austin Donovan");
    }

    #[test]
    fn processor_writes_error_for_missing_variables()
    {
        let dec = Processor::new();
        let view = View::raw("${person}").apply(&dec);

        assert_eq!(view.content, "RefError: `person`");
    }

    #[test]
    fn processor_replaces_positive_conditional()
    {
        let mut dec = Processor::new();
        dec.put("person", Person {
            name: "Austin Donovan".into(),
            age: 19,
        }).unwrap();

        let view = View::raw("@if person { ${person.name} is ${person.age} }")
            .apply(&dec);

        assert_eq!(view.content, "Austin Donovan is 19");
    }

    #[test]
    fn processor_replaces_negative_conditional()
    {
        let dec = Processor::new();
        let view = View::raw("@if person { ${person.name} is ${person.age} }")
            .apply(&dec);

        assert_eq!(view.content, "");
    }

    #[test]
    fn processor_replaces_foreach()
    {
        let mut dec = Processor::new();
        dec.put("people", vec![
            Person {
                name: "Austin Donovan".into(),
                age: 19,
            },
            Person {
                name: "John Doe".into(),
                age: 40,
            },
        ]).unwrap();

        let input = r"@foreach it in people { ${it.name} is ${it.age}. }";
        let view = View::raw(input).apply(&dec);

        assert_eq!(view.content, "Austin Donovan is 19.John Doe is 40.");
    }

    #[test]
    fn processor_replaces_nested()
    {
        let mut dec = Processor::new();
        dec.put("people", vec![
            Person {
                name: "Austin Donovan".into(),
                age: 19,
            },
            Person {
                name: "John Doe".into(),
                age: 40,
            },
        ]).unwrap();

        let input =
            r"@if people {
                  @foreach person in people {
                    ${person.name} is ${person.age}.
                  }
              }";
        let view = View::raw(input).apply(&dec);

        assert_eq!(view.content, "Austin Donovan is 19.John Doe is 40.");
    }

    #[test]
    fn processor_ignores_other_text()
    {
        let mut dec = Processor::new();
        dec.put("person", Person {
            name: "Austin Donovan".into(),
            age: 19,
        }).unwrap();

        let view = View::raw("I'm a person").apply(&dec);

        assert_eq!(view.content, "I'm a person");
    }
}
