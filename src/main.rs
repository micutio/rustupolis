extern crate core;

use crate::server::Server;
use crate::repository::Repository;

mod repository;
mod tcp_server;
mod udp_server;
mod server;
mod lexing;

fn main () {
 let ip_address =String::from("127.0.0.1");
 let port = String::from("9000");

 let repository = Repository::new();

 let new_server = Server::new(server::Protocol::TCP, ip_address,port,repository);
 new_server.start_server()
}