use crate::{Repository, tcp_server, udp_server};

pub enum Protocol{
    TCP,
    UDP
}

pub struct Server{
    protocol: Protocol,
    ip_address: String,
    port: String,
    repository: Repository
}

impl Server {

    pub fn new(protocol: Protocol, ip_address: String, port: String, repository: Repository) -> Server {
        Server {
            protocol,
            ip_address,
            port,
            repository
        }
    }

    pub fn start_server(self) {
         match self.protocol {
            Protocol::TCP => {
                tcp_server::launch_server(&self.ip_address, &self.port, &self.repository).unwrap()
            },
            Protocol::UDP => {
                udp_server::launch_server(&self.ip_address, &self.port, &self.repository).unwrap()
            }
        }
    }
}