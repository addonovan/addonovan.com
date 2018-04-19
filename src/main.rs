extern crate mwf;

mod projects;
mod decorators;

use projects::ProjectController;

use mwf::ServerBuilder;

#[cfg(debug_assertions)]
const ADDR: &'static str = "localhost:8080";

#[cfg(not(debug_assertions))]
const ADDR: &'static str = "0.0.0.0:80";

fn main()
{
    ServerBuilder::new()
        .bind("/projects/:page?", ProjectController::new())
        .address(ADDR)
        .start()
        .unwrap();
}
