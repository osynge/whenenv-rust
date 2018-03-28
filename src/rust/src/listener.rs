use mio::*;
use mio::unix::EventedFd;

use mio::{Events, Poll, PollOpt, Ready, Token};

use mio::timer::Timer;
use std::thread;
use std::io::{self, Error, Read};
use std::collections::HashMap;

use std::os::unix::io::FromRawFd;
use std::fs::File;
use std::os::unix::io::RawFd;
use std::time::Duration;

pub fn henry() -> Result<(), Error> {
    // After this number of sockets is accepted, the server will shutdown.
    const MAX_SOCKETS: usize = 32;

    // Pick a token that will not be used by any other socket and use that one
    // for the listener.
    const LISTENER: Token = Token(1024);
    const TIMEOUT: Token = Token(1);

    // This is used to generate a unique token for a socket
    let mut next_socket_index = 0;

    // The `Poll` instance
    let poll = Poll::new()?;

    // Tcp listener
    let fd: RawFd = 1;
    let listener = EventedFd(&fd);

    let mut timer = Timer::default();
    timer
        .set_timeout(Duration::from_millis(200), "hello")
        .unwrap();

    // Register the listener
    poll.register(&listener, LISTENER, Ready::readable(), PollOpt::edge());
    poll.register(&timer, TIMEOUT, Ready::readable(), PollOpt::edge())
        .unwrap();
    // Spawn a thread that will connect a bunch of sockets then close them

    thread::spawn(move || {
        // +1 here is to connect an extra socket to signal the socket to close
        for _ in 0..(MAX_SOCKETS + 1) {
            // Connect then drop the socket

        }
    });

    let mut bill = unsafe { File::from_raw_fd(0) };
    // Event storage
    let mut events = Events::with_capacity(1024);

    // Read buffer, this will never actually get filled
    let mut buf = [0; 256];

    // The main event loop
    loop {
        // Wait for events
        poll.poll(&mut events, None)?;

        for event in &events {
            match event.token() {
                LISTENER => {
                    // Perform operations in a loop until `WouldBlock` is
                    // encountered.

                    let mut buf: [u8; 1024] = [0; 1024];
                    let readrc = bill.read(&mut buf);

                    let foo = bill.read(&mut buf);
                    match foo {
                        Ok(j) => {
                            println!("{}", j);

                            // Register the new socket w/ poll
                            poll.register(&listener, LISTENER, Ready::readable(), PollOpt::edge());
                        }

                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // Socket is not ready anymore, stop accepting
                            break;
                        }
                        e => panic!("err={:?}", e), // Unexpected error
                    }
                }
                TIMEOUT => {
                    return Ok(());
                }
                token => {}
            }
        }
    }
}
