use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();

    let mut buf_reader = BufReader::new(&stream);

    let _len: usize = buf_reader.read_line(&mut buf)?;

    let first_line: &str = buf.trim();

    if first_line == "GET / HTTP/1.1" {
        stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
    } else if first_line.starts_with("GET /echo/") {
        let message = first_line
            .replace("GET /echo/", "")
            .replace(" HTTP/1.1", "");
        let len = message.len();
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {len}\r\n\r\n{message}").as_bytes())?;
    } else {
        stream.write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes())?;
    }

    Ok(())
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
