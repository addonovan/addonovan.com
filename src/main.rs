extern crate mwf;

use mwf::{ServerBuilder, RequestHandler, View, ViewResult};
use mwf::routing::RouteMap;

#[cfg(debug_assertions)]
const ADDR: &'static str = "localhost:8080";

#[cfg(not(debug_assertions))]
const ADDR: &'static str = "0.0.0.0:80";

struct HelloWorld;
impl RequestHandler for HelloWorld
{
    fn handle(&self, _route_map: RouteMap) -> ViewResult
    {
        View::from(format!("I'm listening on {}!", ADDR))
    }
}

fn main()
{
    ServerBuilder::new()
        .bind("/", HelloWorld {})
        .address(ADDR)
        .start()
        .unwrap();
}
