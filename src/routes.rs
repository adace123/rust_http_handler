use std::net::TcpStream;
use super::http_request::HttpRequest;
use super::http_response::{HttpResponse, Status};

pub struct RequestHandler {}

trait RouteHandler {
    fn handle_route(&self, request: HttpRequest, response: &mut HttpResponse) -> Status;
}

struct HttpUpperCaseHandler {}

impl RouteHandler for HttpUpperCaseHandler {
    fn handle_route(&self, request: HttpRequest, response: &mut HttpResponse) -> Status {
        response.send(Status::Ok, request.body.trim().to_ascii_uppercase())
    }
}

impl RequestHandler {

    pub fn handle_request(&self, stream: &mut TcpStream) {
        let request = HttpRequest::new(stream);
        let response = &mut HttpResponse::new(stream);

        let status: Status = request.map(|req| {
            match req.method.as_str() {
                "POST" | "GET" => {
                    match req.uri.as_str() {
                        "/upper" => {
                            HttpUpperCaseHandler{}.handle_route(req, response)
                        },
                        _ => Status::NotFound
                    }
                },
                _ => Status::MethodNotAllowed
            }
        }).unwrap_or_else(|_| Status::BadRequest);

        if status != Status::Ok {
            response.handle_error(status);
        }

    }

    pub fn new() -> RequestHandler {
        RequestHandler {}
    }

}
