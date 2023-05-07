use crate::client::Client;
use crate::repository::RequestResponse;
use crate::Repository;

use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};

use crate::constant::{CONNECTED, OK, TUPLE_SPACE_ATTACHED, TUPLE_SPACE_ATTACHED_UPDATED};
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::str::from_utf8;

// Setup some tokens to allow us to identify which event is for which socket.
const TCP_TOKEN: Token = Token(0);

#[cfg(not(target_os = "wasi"))]
pub fn launch_server(ip_address: &str, port: &str, repository: &Repository) -> anyhow::Result<()> {
    // Setup the TCP server socket.
    let address = format!("{}:{}", ip_address, port);
    let addr = address.parse()?;
    let mut socket = TcpListener::bind(addr)?;

    // Create a poll instance.
    let mut poll = Poll::new()?;
    poll.registry()
        .register(&mut socket, TCP_TOKEN, Interest::READABLE)?;

    // Create storage for events, clients and connection.
    let mut events = Events::with_capacity(128);
    let mut clients: HashMap<Token, Client> = HashMap::new();
    let mut connections: HashMap<Token, TcpStream> = HashMap::new();

    // Unique token for each incoming connection.
    let mut unique_token = Token(TCP_TOKEN.0 + 1);

    println!("You can connect to the TCP server using `ncat`:");
    println!("ncat {} {}", ip_address, port);

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                TCP_TOKEN => loop {
                    // Received an event for the TCP server socket, which indicates we can accept a
                    // connection.
                    let (mut connection, address) = match socket.accept() {
                        Ok((connection, address)) => (connection, address),
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // If we get a `WouldBlock` error we know our listener has no more
                            // incoming connections queued,
                            // so we can return to polling and wait for some more.
                            break;
                        }
                        Err(_) => {
                            break;
                        }
                    };

                    log::info!("accepted connection from: {}", address);

                    let token = next(&mut unique_token);
                    poll.registry().register(
                        &mut connection,
                        token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )?;

                    connections.insert(token, connection);
                },
                token => {
                    // Maybe received an event for a TCP connection.
                    let done = if let Some(connection) = connections.get_mut(&token) {
                        handle_connection_event(
                            poll.registry(),
                            connection,
                            event,
                            &mut clients,
                            repository,
                        )
                        .unwrap_or(true)
                    } else {
                        // Sporadic events happen, we can safely ignore them.
                        log::debug!("server event: {event:?}");
                        false
                    };

                    if done {
                        if let Some(mut connection) = connections.remove(&token) {
                            poll.registry().deregister(&mut connection)?;
                        }
                    }
                }
            }
        }
    }
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

/// Returns `true` if the connection is done.
fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
    clients: &mut HashMap<Token, Client>,
    repository: &Repository,
) -> io::Result<bool> {
    if event.is_writable() {
        match connection.write(CONNECTED.as_ref()) {
            Ok(n) if n < CONNECTED.len() => return Err(io::ErrorKind::WriteZero.into()),
            Ok(_) => registry.reregister(connection, event.token(), Interest::READABLE)?,
            Err(ref err) if would_block(err) => {}
            Err(ref err) if interrupted(err) => {
                return handle_connection_event(registry, connection, event, clients, repository)
            }
            Err(err) => return Err(err),
        }
    }

    if event.is_readable() {
        let mut connection_closed = false;
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;
        // We can (maybe) read from the connection.
        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                // Other errors we'll consider fatal.
                Err(err) => return Err(err),
            }
        }

        if bytes_read != 0 {
            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {
                let client_option = clients.get(&event.token());
                let client_request = String::from(str_buf.trim_end());
                log::debug!("client request: {}", client_request);

                let result = repository.manage_request(client_request, client_option);
                match result {
                    RequestResponse::SpaceResponse(client) => {
                        match clients.insert(event.token(), client) {
                            None => {
                                if let Err(e) = connection.write(TUPLE_SPACE_ATTACHED.as_ref()) {
                                    log::error!("{e}")
                                }
                            }
                            Some(_) => {
                                if let Err(e) =
                                    connection.write(TUPLE_SPACE_ATTACHED_UPDATED.as_ref())
                                {
                                    log::error!("{e}")
                                }
                            }
                        };
                    }
                    RequestResponse::NoResponse(x) => {
                        if let Err(e) = connection.write(x.as_ref()) {
                            log::error!("{e}")
                        }
                    }
                    RequestResponse::OkResponse() => {
                        if let Err(e) = connection.write(OK.as_ref()) {
                            log::error!("{e}")
                        }
                    }
                    RequestResponse::DataResponse(tuple_list) => {
                        if let Err(e) = connection.write(tuple_list.as_ref()) {
                            log::error!("{e}")
                        }
                    }
                }
            } else {
                log::info!("Received (non-UTF-8) data: {:?}", received_data);
            }
        }

        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }
    Ok(false)
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
