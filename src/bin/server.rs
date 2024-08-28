use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
};

use anyhow::Context;
use http::{Request, Response};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ipv4_address = Ipv4Addr::LOCALHOST;
    let port: u16 = 4221;
    let socket_address: SocketAddr = (ipv4_address, port).into();

    let listener = TcpListener::bind(socket_address)
        .await
        .with_context(|| format!("Failed to bind to socket address: {ipv4_address}:{port}"))?;
    println!("Bound to socket address: {socket_address}");

    loop {
        let (stream, client_socket_address) = listener
            .accept()
            .await
            .context("Failed to accept a new incoming connection")?;
        println!("Accepted incoming connection from: {client_socket_address}");

        loop {
            // https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html#method.try_read
            stream
                .readable()
                .await
                .context("Socket is not readable yet")?;

            let mut buffer = [0u8; 1024];

            match stream.try_read(&mut buffer) {
                Ok(0) => break,
                Ok(number_of_bytes) => println!("Read {number_of_bytes} bytes into buffer"),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(error) => return Err(error).context("Failed to read bytes into buffer"),
            };

            let response = match Request::parse(buffer.as_slice()) {
                Err(error) => {
                    eprintln!("Failed to parse bytes because of error: {error}");
                    Response::internal_server_error().build()
                }
                Ok((_, request)) if request.target() == "/" => {
                    println!("Received request: {request:?}");
                    Response::ok().build()
                }
                Ok((_, request)) if request.target().starts_with("/echo/") => {
                    let target_suffix = request
                        .target()
                        .strip_prefix("/echo/")
                        .expect("We've already checked that this string starts with '/echo/'");
                    println!("Received request: {request:?}");
                    Response::ok().set_body(target_suffix.as_str()).build()
                }
                Ok((_, request)) if request.target() == "/user-agent" => {
                    println!("Received request: {request:?}");
                    let user_agent = request
                        .headers()
                        .user_agent()
                        .expect(
                            "Requests to the '/user-agent' endpoint should have a 'User-Agent' header",
                        )
                        .to_string();
                    Response::ok().set_body(user_agent).build()
                }
                Ok((_, request)) => {
                    println!("Received request: {request:?}");
                    Response::not_found().build()
                }
            };
            println!("Generated response: {response}");

            let response_string = response.to_string();

            // https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html#method.try_write
            stream
                .writable()
                .await
                .context("Socket is not writeable yet")?;

            match stream.try_write(response_string.as_bytes()) {
                Ok(0) => break,
                Ok(number_of_bytes) => println!("Wrote {number_of_bytes} bytes"),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(error) => return Err(error).context("Failed to write bytes to stream"),
            };
        }
    }
}
