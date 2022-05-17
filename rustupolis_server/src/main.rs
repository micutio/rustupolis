extern crate core;

use crate::repository::Repository;
use crate::server::Server;

mod client;
mod constant;
mod lexing;
pub mod repository;
pub mod server;
mod tcp_server;
mod udp_server;

fn main() {
    let ip_address = String::from("127.0.0.1");
    let port_tcp = String::from("9000");
    let port_udp = String::from("9001");

    let repository = Repository::new();

    let server_tcp = Server::new(server::Protocol::TCP, &ip_address, &port_tcp, &repository);
    let server_udp = Server::new(server::Protocol::UDP, &ip_address, &port_udp, &repository);

    let server_list = vec![server_tcp, server_udp];
        crossbeam::scope(|scope| {
        for server in server_list {
            scope.spawn(move |_| match server.start_server() {
                Ok(_) => {
                    println!("{}", "OK ")
                }
                Err(error) => {
                    println!("{}", error)
                }
            });
        }
    })
    .unwrap();
}
