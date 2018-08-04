extern crate actix_web;
extern crate handlebars;
extern crate serde;
extern crate chrono;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

use actix_web::App;
use actix_web::http::Method;
use actix_web::server;

mod controller;
mod decorator;
mod file_cache;

use controller::Controller;

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

    server::new(factory)
        .bind("0.0.0.0:8080").expect("Failed to bind server to address")
        .run();
}
