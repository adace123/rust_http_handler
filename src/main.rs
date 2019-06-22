#![allow(dead_code)]
mod http_mod;
use std::net::{TcpListener, TcpStream};
use std::thread;
use http_mod::HttpRequest;

fn main() {
    let server = TcpListener::bind("127.0.0.1:1024").expect("Could not bind");
    
    for stream in server.incoming() {
       thread::spawn(move || {
           let mut stream: TcpStream = stream.unwrap();
           handle_request(&mut stream);
       });
    }

}

fn handle_request(stream: &mut TcpStream) {
    let request: HttpRequest = HttpRequest::new(stream);
    match request.method.as_str() {
        "GET" => {

        },
        "POST" => {

        },
        other => {
            panic!(format!("Method {} not supported", other));
        }
    };
    // stream.write_all(body.to_uppercase().as_bytes()).expect("Could not write");
}
