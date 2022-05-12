use crate::{tcp_server, udp_server, Repository};

pub enum Protocol {
    TCP,
    UDP,
}

pub struct Server<'a> {
    protocol:   Protocol,
    ip_address: &'a String,
    port:       &'a String,
    repository: &'a Repository,
}

impl Server<'_> {
    pub fn new<'a>(
        protocol: Protocol,
        ip_address: &'a String,
        port: &'a String,
        repository: &'a Repository,
    ) -> Server<'a> {
        Server {
            protocol,
            ip_address,
            port,
            repository,
        }
    }

    pub fn start_server(self) -> std::io::Result<()> {
        match self.protocol {
            Protocol::TCP => {
                tcp_server::launch_server(&self.ip_address, &self.port, &self.repository)
            }
            Protocol::UDP => {
                udp_server::launch_server(&self.ip_address, &self.port, &self.repository)
            }
        }
    }
}
