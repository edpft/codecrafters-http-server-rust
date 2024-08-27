mod body;
mod headers;
mod status_line;

use std::{fmt, io::Write, net::TcpListener};

use body::Body;
use headers::Headers;
use status_line::StatusLine;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let default_response_string = HttpResponse::default().to_string();
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

#[derive(Clone, Debug, Default)]
struct HttpResponse {
    status_line: StatusLine,
    headers: Headers,
    body: Option<Body>,
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.body {
            None => write!(f, "{}{}", self.status_line, self.headers),
            Some(body) => write!(f, "{}{}{}", self.status_line, self.headers, body),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_response() {
        let default_response = HttpResponse::default();
        assert_eq!(
            default_response.to_string(),
            String::from("HTTP/1.1 200 OK\r\n\r\n")
        );
    }
}
