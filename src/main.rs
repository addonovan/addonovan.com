extern crate mwf;

extern crate regex;
extern crate chrono;

#[macro_use] extern crate lazy_static;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use mwf::ServerBuilder;

mod projects;
mod config;
mod decs;

use projects::ProjectController;
use config::CONFIG;

fn main()
{
    ServerBuilder::new()
        .bind("/projects/:page?", ProjectController::new())
        .addr(CONFIG.address.parse().unwrap())
        .start();
}
