extern crate regex;
mod http_uppercase;
mod time;
pub mod common;

use std::net::TcpStream;
use super::http_request::HttpRequest;
use super::http_response::{HttpResponse, Status};
use http_uppercase::HttpUpperCaseHandler;
use time::Time;
use regex::Regex;
use log::info;

pub use common::*;

impl RequestHandler {

    pub fn handle_request(&self, stream: &mut TcpStream) -> Status {
        let request = HttpRequest::new(stream);
        let response = &mut HttpResponse::new(stream);
        let status = request.as_ref().map(|req| {
            info!("{} {}", req.method, req.uri);
            let status = match req.method.as_str() {
                "POST" if !req.is_empty_body() => {
                    match req.uri.as_str()  {
                        "/upper" => {
                            HttpUpperCaseHandler::new().handle_route(req, response)
                        },
                        
                        _ => Status::NotFound
                    }
                },
                "POST" => Status::NoContent,
                "GET" => {
                    if Regex::new("^/api/(unixtime|parsetime)").unwrap().is_match(req.uri.as_str()) {
                        return Time::new().handle_route(req, response)
                    } 
                    Status::NotFound
                }
                _ => Status::MethodNotAllowed
            };

            status
        }).unwrap_or_else(|_| {
            Status::BadRequest
        });
        
        if status != Status::Ok {
            response.handle_error(status);
        }

        status

    }

    pub fn new() -> Self {
        RequestHandler {}
    }

}
