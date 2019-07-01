#![allow(dead_code)]
mod http_request;
mod http_response;
mod routes;
use std::net::{TcpListener, TcpStream};
use std::thread;
use routes::RequestHandler;

fn main() {
    let server = TcpListener::bind("127.0.0.1:1024").expect("Could not bind");
    println!("Server started");
    for stream in server.incoming() {
       thread::spawn(move || {
           let handler = RequestHandler::new();
           let mut stream: TcpStream = stream.unwrap();
           handler.handle_request(&mut stream);
       });
    }

}
