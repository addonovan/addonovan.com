
use actix_web::{HttpRequest, HttpResponse};

pub trait Controller
{
    fn handle(&self, req: &HttpRequest) -> HttpResponse;
}
