use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

use anyhow::Context;
use http::{Request, Response, ThreadPool};

fn main() -> anyhow::Result<()> {
    let ipv4_address = Ipv4Addr::LOCALHOST;
    let port: u16 = 4221;
    let socket_address: SocketAddr = (ipv4_address, port).into();

    let listener = TcpListener::bind(socket_address)
        .with_context(|| format!("Failed to bind to socket address: {ipv4_address}:{port}"))?;
    println!("Bound to socket address: {socket_address}");
    let pool = ThreadPool::new(4);

    // ? Should I ignore errors here?
    for stream in listener.incoming() {
        match stream {
            Err(error) => eprintln!("Connection failed because of error: {error}"),
            Ok(stream) => {
                pool.execute(|| {
                    let _ = handle_connection(stream);
                });
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buffer = [0u8; 1024];

    match stream.read(&mut buffer) {
        Ok(number_of_bytes) => println!("Read {number_of_bytes} bytes into buffer"),
        Err(error) => return Err(error).context("Failed to read bytes into buffer"),
    };

    let response_string = generate_response(buffer.as_slice()).to_string();

    match stream.write(response_string.as_bytes()) {
        Ok(number_of_bytes) => println!("Wrote {number_of_bytes} bytes"),
        Err(error) => return Err(error).context("Failed to write bytes to stream"),
    };

    Ok(())
}

fn generate_response(bytes: &[u8]) -> Response {
    let response = match Request::parse(bytes) {
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
                .expect("Requests to the '/user-agent' endpoint should have a 'User-Agent' header")
                .to_string();
            Response::ok().set_body(user_agent).build()
        }
        Ok((_, request)) => {
            println!("Received request: {request:?}");
            Response::not_found().build()
        }
    };
    println!("Generated response: {response}");
    response
}
