extern crate chrono;
extern crate regex;

use std::net::TcpStream;
use std::io::{Write};
use chrono::Utc;

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Ok,
    NotFound,
    BadRequest,
    MethodNotAllowed
}

#[derive(Debug)]
pub struct HttpResponse<'a> {
    stream: &'a TcpStream,
    response_headers: String
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Status::Ok => write!(f, "200 OK"),
            Status::NotFound => write!(f, "404 Not Found"),
            Status::BadRequest => write!(f, "400 Bad Request"),
            Status::MethodNotAllowed => write!(f, "405 Method Not Allowed")
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(stream: &TcpStream) -> HttpResponse {
        HttpResponse {
            stream: stream,
            response_headers: HttpResponse::get_headers()
        }
    }

    fn get_headers() -> String {
        let formatted_date = Utc::now().format("%a, %d %h %Y %H:%M:%S %Z").to_string();
        format!("Date: {}\r\nServer: Rust\r\nContent-Type: application/text\r\nConnection: Closed\r\n\r\n", &formatted_date)
    }

    pub fn send(&mut self, status: Status, content: String) {
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n{}{}", 
            status, 
            content.as_bytes().len(),
            self.response_headers, 
            content
        );
        write!(self.stream, "{}", response.trim()).unwrap();
    }
}
