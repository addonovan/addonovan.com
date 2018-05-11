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
mod services;

use projects::ProjectController;
use config::CONFIG;
use services::ServiceController;

fn main()
{
    ServerBuilder::new()
        .bind("/projects/:page?", ProjectController::new())
        .bind("/services/:page?", ServiceController::new())
        .addr(CONFIG.address.parse().unwrap())
        .start();
}
