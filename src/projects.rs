use mwf::{ViewResult, View, ViewDecorator, RequestHandler};
use mwf::routing::RouteMap;

use super::decorators::SurroundDecorator;

pub struct ProjectController
{
    format: SurroundDecorator,
}

impl ProjectController
{
    /// Creates a new project controller
    pub fn new() -> Self
    {
        ProjectController {
            format: SurroundDecorator::file("res/projects.html").unwrap(),
        }
    }
}

impl RequestHandler for ProjectController
{
    fn handle(&self, _route_map: RouteMap) -> ViewResult
    {
        View::from("Hello!").and_then(|x| {
            self.format.decorate(x)
        })
    }
}
