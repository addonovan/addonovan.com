extern crate mwf;

use mwf::ServerBuilder;

mod projects;
mod config;
use projects::ProjectController;
use config::CONFIG;

fn main()
{
    ServerBuilder::new()
        .bind("/projects/:page?", ProjectController::new())
        .addr(CONFIG.address.parse().unwrap())
        .start();
}
