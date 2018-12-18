extern crate actix_web;
extern crate chrono;
extern crate handlebars;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use actix_web::App;
use actix_web::http::Method;
use actix_web::server;

mod cache;
mod constants;
mod controller;
mod decorator;
mod template;
mod util;

use controller::Controller;
use constants::CONFIG;

mod controllers
{
    use controller::*;
    use constants::{EXPERIMENT_DIR, RAW_DIR};

    lazy_static! {
        pub static ref BLOG: BlogController = BlogController::new();
        pub static ref MAIN: MainController = MainController::new();
        pub static ref RAW: Raw = Raw::new(RAW_DIR);
        pub static ref EXPERIMENT: Raw = Raw::new(EXPERIMENT_DIR);
        pub static ref HOME: Home = Home::new();
    }

}

fn main() {
    let factory = || { vec![
        App::new()
            .resource("/raw/{tail:.*}", |r| {
                r.method(Method::GET).f(|r| controllers::RAW.handle(r))
            })
            .resource("/experiment/{tail:.*}", |r| {
                r.method(Method::GET).f(|r| controllers::EXPERIMENT.handle(r))
            })
            .resource("/cache_overview", |r| {
                r.method(Method::GET).f(|r| controllers::MAIN.cache_overview(r))
            })
            .resource("/blog/{year:.*}/{month:.*}/{day:.*}", |r| {
                r.method(Method::GET).f(|r| controllers::BLOG.handle(r))
            })
            .resource("/blog", |r| {
                r.method(Method::GET).f(|r| controllers::BLOG.overview(r))
            })
            .resource("/", |r| {
                r.method(Method::GET).f(|r| controllers::HOME.handle(r))
            })
            .resource("/{tail:.*}", |r| {
                r.method(Method::GET).f(|r| controllers::MAIN.handle(r))
            })
            .finish(),
    ] };

    println!("Binding to address: {}", CONFIG.bind_address);
    server::new(factory)
        .bind(CONFIG.bind_address).expect("Failed to bind server to address")
        .run();
}
