use super::http_request::HttpRequest;
use super::http_response::{HttpResponse, Status};

pub trait RequestHandler {
    fn handle_request(&self, request: HttpRequest, response: &mut HttpResponse);

    fn handle_error(&self, status: Status, response: &mut HttpResponse) {
        response.send(status, format!("Error {}", status));        
    }
}

struct HttpUpperCaseHandler {}

struct BadRequestHandler {}

struct MethodNotSupported {}

struct NotFound {}

impl RequestHandler for HttpUpperCaseHandler {
    fn handle_request(&self, request: HttpRequest, response: &mut HttpResponse) {
        response.send(Status::Ok, request.body.to_ascii_uppercase());
    }
}

impl RequestHandler for BadRequestHandler {
    fn handle_request(&self, request: HttpRequest, response: &mut HttpResponse) {
        response.send(Status::BadRequest, format!("Error {}", Status::BadRequest));
    }
}

impl RequestHandler for MethodNotSupported {
    fn handle_request(&self, request: HttpRequest, response: &mut HttpResponse) {
        response.send(Status::MethodNotAllowed, format!("Error {}", Status::MethodNotAllowed));
    }
}

impl RequestHandler for NotFound {
    fn handle_request(&self, request: HttpRequest, response: &mut HttpResponse) {
        response.send(Status::NotFound, format!("Error {}", Status::NotFound));
    }
}

pub fn route_handler(route: &str, method: &str) -> Box<RequestHandler> {
    match method {
        "POST" => {
            match route {
                "/upper" => Box::new(HttpUpperCaseHandler{}),
                _ => Box::new(NotFound{})
            }
        },
        _ => Box::new(MethodNotSupported{})
    }
}
