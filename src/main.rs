extern crate actix_web;
extern crate chrono;
extern crate handlebars;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use] extern crate serde_derive;

use actix_web::App;
use actix_web::http::Method;
use actix_web::server;

mod cache;
mod constants;
mod controller;
mod decorator;

use controller::Controller;
use constants::CONFIG;

mod controllers
{
    use controller::*;

    lazy_static! {
        pub static ref MAIN: MainController = MainController::new();

        pub static ref RAW: Raw = Raw::new();
    }

}

fn main() {
    let factory = || { vec![
        App::new()
            .prefix("/raw")
            .resource("/{tail:.*}", |r| {
                r.method(Method::GET).f(|r| controllers::RAW.handle(r))
            })
            .finish(),

        App::new()
            .resource("/cache_overview", |r| {
                r.method(Method::GET).f(|r| controllers::MAIN.cache_overview(r))
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
