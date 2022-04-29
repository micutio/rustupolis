extern crate core;

use crate::repository::Repository;
use crate::server::Server;

mod lexing;
pub mod repository;
pub mod server;
mod tcp_server;
mod udp_server;

fn main() {
    let ip_address = String::from("127.0.0.1");
    let port_tcp = String::from("9000");
    //let port_udp = String::from("9001");

    let repository = Repository::new();

    let new_server = Server::new(server::Protocol::TCP, &ip_address, &port_tcp, &repository);
    // let new_server2 = Server::new(server::Protocol::UDP, &ip_address,&port_udp,&repository);

    match new_server.start_server() {
        Ok(_) => {
            println!("{}", "OK ")
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}
