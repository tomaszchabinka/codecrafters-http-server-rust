use crate::pool::ThreadPool;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

pub mod pool;

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut content = vec![];
    let mut buf_reader = BufReader::new(&stream);

    loop {
        let mut buf = String::new();
        let _len: usize = buf_reader.read_line(&mut buf)?;

        if &buf == "\r\n" {
            break;
        }

        content.push(buf);
    }

    let first_line: &str = content[0].trim();

    if first_line == "GET / HTTP/1.1" {
        stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
    } else if first_line.starts_with("GET /files/") {
        let args: Vec<String> = env::args().collect();

        let directory = args.last().unwrap();

        let filename = first_line
            .replace("GET /files/", "")
            .replace(" HTTP/1.1", "");

        if let Ok(mut file) = File::open(format!("{directory}{filename}")) {
            let mut content = vec![];
            let len = file.read_to_end(&mut content)?;

            println!("{}", len);

            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {len}\r\n\r\n").as_bytes())?;
            stream.write_all(&content)?;
        } else {
            stream.write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes())?;
        }
    } else if first_line.starts_with("GET /echo/") {
        let message = first_line
            .replace("GET /echo/", "")
            .replace(" HTTP/1.1", "");
        let len = message.len();
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {len}\r\n\r\n{message}").as_bytes())?;
    } else if first_line.starts_with("GET /user-agent") {
        let message = content
            .iter()
            .find(|line| line.starts_with("User-Agent: "))
            .unwrap()
            .replace("User-Agent: ", "");
        let len = message.trim().len();
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {len}\r\n\r\n{message}").as_bytes())?;
    } else if first_line.starts_with("POST /files") {
        let args: Vec<String> = env::args().collect();

        let directory = args.last().unwrap();

        let filename = first_line
            .replace("POST /files/", "")
            .replace(" HTTP/1.1", "");

        let size = content
            .iter()
            .find(|line| line.starts_with("Content-Length: "))
            .unwrap()
            .replace("Content-Length: ", "")
            .trim()
            .parse::<usize>()
            .unwrap();

        let mut buffer = vec![0; size];
        buf_reader.read_exact(&mut buffer).unwrap();

        std::fs::write(format!("{directory}{filename}"), &buffer)?;

        stream.write_all("HTTP/1.1 201 CREATED\r\n\r\n".as_bytes())?;
    } else {
        stream.write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes())?;
    }

    println!("Done!");

    Ok(())
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                let _ = handle_client(stream);
            }),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
