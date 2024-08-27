use std::{io::Write, net::TcpListener};

use http_server::http::Response;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let default_response_string = Response::default().to_string();
                match stream.write_all(default_response_string.as_bytes()) {
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
