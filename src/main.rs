#![allow(dead_code)]
mod http_request;
mod http_response;
mod routes;
use std::net::{TcpListener, TcpStream};
use std::thread;
use http_request::HttpRequest;
use http_response::HttpResponse;
use routes::route_handler;

fn main() {
    let server = TcpListener::bind("127.0.0.1:1024").expect("Could not bind");
    println!("Server started");
    for stream in server.incoming() {
       thread::spawn(move || {
           let mut stream: TcpStream = stream.unwrap();
           handle_request(&mut stream);
       });
    }

}


fn handle_request(stream: &mut TcpStream) {
    let request = HttpRequest::new(stream);
    let response = &mut HttpResponse::new(stream);
    let handler = route_handler(&request.uri, &request.method);
    handler.handle_request(request, response);    
}
