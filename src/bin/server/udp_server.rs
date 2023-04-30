use crate::client::Client;
use crate::constant::{OK, TUPLE_SPACE_ATTACHED, TUPLE_SPACE_ATTACHED_UPDATED};
use crate::repository::RequestResponse;
use crate::Repository;
use log::warn;
use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::str::from_utf8;

// A token to allow us to identify which event is for the `UdpSocket`.
const UDP_SOCKET: Token = Token(0);

#[cfg(not(target_os = "wasi"))]
pub(crate) fn launch_server(
    ip_address: &String,
    port: &String,
    repository: &Repository,
) -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(126);

    let address = format!("{}:{}", ip_address, port);

    // Setup the UDP server socket.
    let addr = address.parse().unwrap();
    let mut socket = UdpSocket::bind(addr)?;
    let mut client_list: HashMap<SocketAddr, Client> = HashMap::new();

    // Register our socket with the token defined above and an interest in being
    // `READABLE`.
    poll.registry()
        .register(&mut socket, UDP_SOCKET, Interest::READABLE)?;

    println!("You can connect to the UDP server using `ncat`:");
    println!("ncat -u {} {}", ip_address, port);

    let mut buf = [0; 1 << 16];

    loop {
        // Poll to check if we have events waiting for us.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // Validate the token we registered our socket with, in this example it will only ever
            // be one but we make sure it's valid nonetheless.
            match event.token() {
                UDP_SOCKET => loop {
                    match socket.recv_from(&mut buf) {
                        Ok((packet_size, source_address)) => {
                            if let Ok(str_buf) = from_utf8(&buf[..packet_size]) {
                                let client = client_list.get(&source_address);
                                let result = repository
                                    .manage_request(String::from(str_buf.trim_end()), client);
                                match result {
                                    RequestResponse::SpaceResponse(new_client) => {
                                        match client_list.insert(source_address, new_client) {
                                            None => {
                                                if let Err(e) = socket.send_to(
                                                    TUPLE_SPACE_ATTACHED.as_ref(),
                                                    source_address,
                                                ) {
                                                    log::error!("{e}")
                                                }
                                            }
                                            Some(_) => {
                                                if let Err(e) = socket.send_to(
                                                    TUPLE_SPACE_ATTACHED_UPDATED.as_ref(),
                                                    source_address,
                                                ) {
                                                    log::error!("{e}")
                                                }
                                            }
                                        };
                                    }
                                    RequestResponse::NoResponse(x) => {
                                        if let Err(e) = socket.send_to(x.as_ref(), source_address) {
                                            log::error!("{e}")
                                        }
                                    }
                                    RequestResponse::OkResponse() => {
                                        if let Err(e) = socket.send_to(OK.as_ref(), source_address)
                                        {
                                            log::error!("{e}")
                                        }
                                    }
                                    RequestResponse::DataResponse(tuple_list) => {
                                        if let Err(e) =
                                            socket.send_to(tuple_list.as_ref(), source_address)
                                        {
                                            log::error!("{e}")
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
