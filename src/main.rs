#![allow(dead_code)]
extern crate env_logger;
extern crate log;
mod http_request;
mod http_response;
mod routes;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env;
use std::env::set_var;
use routes::RequestHandler;
use env_logger::{Builder, Target};
use log::info;

fn main() {
    set_var("RUST_LOG", "debug");
    Builder::new()
        .parse_filters(&env::var("RUST_LOG").unwrap_or_default())
        .target(Target::Stdout)
        .init();
    
    let server = TcpListener::bind("127.0.0.1:1024").map_err(|err| {
        panic!(format!("Could not start server: {:?}", err))
    }).unwrap();
    info!("Server started");
    
    for stream in server.incoming() {
       thread::spawn(move || {
           let handler = RequestHandler::new();
           let mut stream: TcpStream = stream.unwrap();
           handler.handle_request(&mut stream);
       });
    }

}
