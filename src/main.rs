extern crate mwf;

mod projects;

use projects::ProjectController;

use mwf::ServerBuilder;

#[cfg(debug_assertions)]
const ADDR: &'static str = "127.0.0.1:8080";

#[cfg(not(debug_assertions))]
const ADDR: &'static str = "0.0.0.0:80";

fn main()
{
    ServerBuilder::new()
        .bind("/projects/:page?", ProjectController::new())
        .addr(ADDR.parse().unwrap())
        .start();
}
