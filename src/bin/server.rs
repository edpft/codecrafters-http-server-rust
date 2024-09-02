use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{env, fs};

use anyhow::Context;
use http::{Request, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::runtime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && &args[1] == "--directory" {
        let value = &args[2];
        env::set_var("DIRECTORY", value);
    } else {
        let value = "/tmp";
        env::set_var("DIRECTORY", value);
    };

    let ipv4_address = Ipv4Addr::LOCALHOST;
    let port: u16 = 4221;
    let server_address: SocketAddr = (ipv4_address, port).into();

    let listener = TcpListener::bind(server_address)
        .await
        .with_context(|| format!("Failed to bind to socket address: {ipv4_address}:{port}"))?;
    println!("Server bound to address: {server_address}");

    let runtime = runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .build()
        .context("Attempting to build `tokio` runtime with 4 workers")?;

    loop {
        let (mut stream, client_address) = listener
            .accept()
            .await
            .context("Failed to accept new connection")?;
        println!("Accepted connection from client at address: {client_address}");

        runtime.spawn(async move {
            let mut buffer = [0u8; 1024];
            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => println!("No bytes to read"),
                    Ok(number_of_bytes) => println!("Read {number_of_bytes} bytes into buffer"),
                    Err(error) => {
                        eprintln!("Failed to read bytes because of error: {error}");
                    }
                };

                let directory =
                    env::var("DIRECTORY").expect("`DIRECTORY` environment variable has been set");

                let response = match Request::parse(buffer.as_slice()) {
                    Err(error) => {
                        eprintln!("Failed to parse bytes because of error: {error}");
                        Response::internal_server_error().build()
                    }
                    Ok((_, request)) => generate_response(request, directory),
                };

                let response_string = response.to_string();

                match stream.write(response_string.as_bytes()).await {
                    Ok(0) => println!("No bytes to write"),
                    Ok(number_of_bytes) => println!("Wrote {number_of_bytes} bytes"),
                    Err(error) => eprintln!("Failed to write bytes because of error: {error}"),
                };
            }
        });
    }
}

fn generate_response(request: Request, directory: String) -> Response {
    let response_builder = match request {
        request if request.target() == "/" => {
            println!("Received request: {request:?}");
            Response::ok()
        }
        request if request.target().starts_with("/echo/") => {
            let target_suffix = request
                .target()
                .strip_prefix("/echo/")
                .expect("We've already checked that this string starts with '/echo/'");
            println!("Received request: {request:?}");
            Response::ok().set_body(target_suffix.as_str())
        }
        request if request.target().starts_with("/files/") => {
            let requested_file_name = request
                .target()
                .strip_prefix("/files/")
                .expect("We've already checked that this string starts with '/files/'");
            let requested_path = Path::new(&directory).join(requested_file_name.as_str());
            match fs::read(requested_path) {
                Ok(content) => Response::ok().set_body(content),
                Err(error) if error.kind() == ErrorKind::NotFound => Response::not_found(),
                Err(error) => {
                    eprintln!(
                        "While trying to read from path: {}",
                        requested_file_name.as_str()
                    );
                    eprintln!("Encountered error: {error}");
                    Response::internal_server_error()
                }
            }
        }
        request if request.target() == "/user-agent" => {
            println!("Received request: {request:?}");
            let user_agent = request
                .headers()
                .user_agent()
                .expect("Requests to the '/user-agent' endpoint should have a 'User-Agent' header")
                .to_string();
            Response::ok().set_body(user_agent)
        }
        request => {
            println!("Received request: {request:?}");
            Response::not_found()
        }
    };
    let response = response_builder.build();
    println!("Generated response: {response}");
    response
}
