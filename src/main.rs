use std::{
    io::{Read, Write},
    net::TcpListener,
};

use http_server::{request::Request, response::Response};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 1024];
                match stream.read(buffer.as_mut()) {
                    Ok(number_of_bytes) => println!("Read {number_of_bytes} bytes"),
                    Err(error) => eprintln!("Failed to read bytes because of error: {error}"),
                };

                let response = match Request::try_from(buffer.as_slice()) {
                    Err(error) => {
                        eprintln!("Failed to parse bytes because of error: {error}");
                        Response::internal_server_error().build()
                    }
                    Ok(request) if request.target() == "/" => {
                        println!("Received request: {request:?}");
                        Response::ok().build()
                    }
                    Ok(request) if request.target().starts_with("/echo/") => {
                        let target_suffix = request
                            .target()
                            .strip_prefix("/echo/")
                            .expect("We've already checked that this string starts with '/echo/'");
                        println!("Received request: {request:?}");
                        Response::ok().set_body(target_suffix).build()
                    }
                    Ok(request) => {
                        println!("Received request: {request:?}");
                        Response::not_found().build()
                    }
                };
                println!("Generated response: {response}");

                let response_string = response.to_string();
                match stream.write_all(response_string.as_bytes()) {
                    Ok(()) => println!("Wrote bytes"),
                    Err(error) => eprintln!("Failed to write bytes because of error: {error}"),
                };
            }
            Err(e) => {
                println!("error: {e}");
            }
        }
    }
}
