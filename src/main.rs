use std::io::Error;
use clap::Parser;

const HOST_NOT_FOUND: i32 = 11001;

/// Simple program to greet a person
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target to attempt reaching. Can be an IP or a domain name.
    destination: String,

    /// Port to attempt connection to.
    port: u16,
}

fn main() {
    let user_args = Args::parse();

    let dest: String = user_args.destination;
    let port: u16 = user_args.port;

    println!("Target: {}", dest.clone());
    println!("Target Port: {}", port.clone());

    // Attempt a connection. If it succeeds, return.
    let connection = std::net::TcpStream::connect((dest.clone(), port.clone()));

    match connection {
        Ok(socket) => {
            println!("Successfully connected to server. Connection details {:?}", socket);
            return;
        }
        Err(e) => {
            debug_connection_error(dest, port, e);
        }
    }
}

fn debug_connection_error(dest: String, port: u16, err: Error) {
    match err.kind() {
        std::io::ErrorKind::ConnectionRefused => {
            println!("
                Connection was actively refused by the server.
                This means the server is up and reachable, but the port is not reachable.
                This could be due to a firewall, or the server not listening on that port.

                Next steps:
                1. Take packet captures on the server using Wireshark.
                2. Ensure the server is listening on the correct port.
                3. Ensure the server is not behind a firewall."
            );
        }
        std::io::ErrorKind::TimedOut => {
            println!("
                Connection timed out. This could be a lot of things:
                1. Server is down.
                1. Server is up, but silently refused the connection.
                2. Server is behind a firewall.

                Next steps:
                1. Take packet capture on the server using tcpdump.
                2. Validate DNS resolution is correct using nslookup."
            );
        }
        _ => {
            // Error kind is not classified yet. Do matching based on OS code.
            match err.raw_os_error() {
                Some(HOST_NOT_FOUND) => {
                    println!("Could not resolve {dest}. Exiting.");
                }
                _ => {
                    println!("Unknown error {:?} occurred when connecting to {dest}:{port}. Exiting.", err);
                }
            }
        }
    }
}