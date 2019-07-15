use super::RouteHandler;
use super::super::http_request::HttpRequest;
use super::super::http_response::{Status, HttpResponse};

pub struct HttpUpperCaseHandler {}

impl HttpUpperCaseHandler {
    pub fn new() -> Self {
        HttpUpperCaseHandler{}
    }
}

impl RouteHandler for HttpUpperCaseHandler {
    fn handle_route(&self, request: &HttpRequest, response: &mut HttpResponse) -> Status {
        response.send(Status::Ok, request.body.trim().to_ascii_uppercase())
    }
}
