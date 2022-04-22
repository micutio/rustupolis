use std::collections::HashMap;
// You can run this example from the root of the mio repo:
// cargo run --example udp_server --features="os-poll net"
use crate::repository::RequestResponse;
use crate::Repository;
use log::warn;
use mio::{Events, Interest, Poll, Token};
use pretty_env_logger::env_logger;
use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use std::io;
use std::net::SocketAddr;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

// A token to allow us to identify which event is for the `UdpSocket`.
const UDP_SOCKET: Token = Token(0);

#[cfg(not(target_os = "wasi"))]
pub(crate) fn launch_server(
    ip_address: &String,
    port: &String,
    repository: &Repository,
) -> io::Result<()> {
    use mio::net::UdpSocket;
    env_logger::init();
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(126);

    let address = format!("{}:{}", ip_address, port);
    // Setup the UDP server socket.
    let addr = address.parse().unwrap();

    let mut socket = UdpSocket::bind(addr)?;

    let mut clients: HashMap<SocketAddr, &Arc<Mutex<Space<SimpleStore>>>> = HashMap::new();
    // Register our socket with the token defined above and an interest in being
    // `READABLE`.
    poll.registry()
        .register(&mut socket, UDP_SOCKET, Interest::READABLE)?;

    println!("You can connect to the server using `ncat`:");
    println!("ncat -u {} {}", ip_address, port);

    let mut buf = [0; 1 << 16];

    loop {
        // Poll to check if we have events waiting for us.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // Validate the token we registered our socket with, in this example it will only ever be one but we
            // make sure it's valid none the less.
            match event.token() {
                UDP_SOCKET => loop {
                    match socket.recv_from(&mut buf) {
                        Ok((packet_size, source_address)) => {
                            if let Ok(str_buf) = from_utf8(&buf[..packet_size]) {
                                let tuple_s_attached = clients.get(&source_address);
                                let result = repository.manage_request(
                                    String::from(str_buf.trim_end()),
                                    tuple_s_attached,
                                );
                                match result {
                                    RequestResponse::SpaceResponse(x) => match x {
                                        None => {
                                            println!("Tuple space not Found")
                                        },
                                        Some(y) => {
                                            match clients.insert(source_address, y) {
                                                None => {
                                                    println!("Tuple space attached")
                                                }
                                                Some(_) => {
                                                    *clients.get_mut(&source_address).unwrap() =
                                                        x.unwrap();
                                                    println!("Tuple space attach updated")
                                                }
                                            }
                                        }
                                    },
                                    RequestResponse::NoResponse(x) => {
                                        if let Err(e) = socket.send_to(x.as_ref(), source_address) {
                                            println!("{}", e)
                                        }
                                    }
                                    RequestResponse::OkResponse() => {
                                        if let Err(e) = socket
                                            .send_to("Operation done\n".as_ref(), source_address)
                                        {
                                            println!("{}", e)
                                        }
                                    }
                                    RequestResponse::DataResponse(x) => {
                                        if let Err(e) =
                                            socket.send_to(x.to_string().as_ref(), source_address)
                                        {
                                            println!("{}", e)
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // If we get a `WouldBlock` error we know our socket
                            // has no more packets queued, so we can return to
                            // polling and wait for some more.
                            break;
                        }
                        Err(e) => {
                            // If it was any other kind of error, something went
                            // wrong and we terminate with an error.
                            return Err(e);
                        }
                    }
                },
                _ => {
                    // This should never happen as we only registered our
                    // `UdpSocket` using the `UDP_SOCKET` token, but if it ever
                    // does we'll log it.
                    warn!("Got event for unexpected token: {:?}", event);
                }
            }
        }
    }
}

#[cfg(target_os = "wasi")]
fn main() {
    panic!("can't bind to an address with wasi")
}
