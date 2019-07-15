use super::super::http_request::HttpRequest;
use super::super::http_response::{HttpResponse, Status};

pub struct RequestHandler {}

pub trait RouteHandler {
    fn handle_route(&self, request: &HttpRequest, response: &mut HttpResponse) -> Status;
}
