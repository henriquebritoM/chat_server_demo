
use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use crate::test_tcp::test_connection;

mod test_tcp;

fn handle_client(mut stream: TcpStream) {
    let mut request: Request;

    match Request::new(stream) {
        Ok(req) => request = req,
        Err(e) => {
            eprintln!("some error ocurred: {}", e);
            return;
        }
    }

    match request.handle() {
        Ok(_) => {},
        Err(e) => eprintln!("some error ocurred: {}", e),
    }
}

struct Request {
    stream: TcpStream,
    method: String,
    path: String,
    protocol: String,
}

impl Request {
    fn new(stream: TcpStream) -> std::io::Result<Request> {
        let mut request = Request {
            stream: stream,
            method: String::new(),
            path: String::new(),
            protocol: String::new()
        };

        let mut request_first_line: String = String::new();
        let mut request_rest: String = String::new();

        match request.read_request()?.split_once("\r\n\r\n") {
            None => {},
            Some((str1, str2)) => {
                request_first_line.push_str(str1);
                request_rest.push_str(str2);
            }
        }

        match request_first_line.split_once(" ") {
            None => {},
            Some((str1, str2)) => {
                request.method.push_str(str1);

                match str2.split_once(" ") {
                    None => {},
                    Some((str2, str3)) => {
                        request.path.push_str(str2);
                        request.protocol.push_str(str3);
                    }
                }
            }
        }

        Ok(request)
    }
    
    fn read_request(&mut self) -> std::io::Result<String> {
        
        let mut request: String = String::new();
        let buf_reader = BufReader::new(&self.stream);
        
        for str in buf_reader.lines() {
            let str = str?;
            if str.is_empty() {break;}
            request.push_str(&format!("{}\r\n\r\n", str));
        }
    
        println!("Request: {:#?}", request);
    
        Ok(request)
    }

    fn handle(&mut self) -> std::io::Result<()> {
        let mut target: &str = "";
        let mut content: &str = "";
        let mut response: String = String::new();
        
        match self.path.rsplit_once("/") {
            None => {},
            Some((str1, str2)) => {
                content = str2;
                target = str1;
            }
        }

        println!("path: {}, target: {}, content: {}", self.path, target, content);

        match target {
            "" => response = "HTTP/1.1 200 OK\r\n\r\n".to_string(),
            "/echo" => response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-length: {}\r\n\r\n{}", content.len(), content).to_string(),
            _ => response = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
        }

        println!("Response: {:#?}", response);
        self.stream.write_all(response.as_bytes())?;

        Ok(())
    }
    
}

fn main() -> std::io::Result<()> {
    
    println!("Hello, world!");

    let port = portpicker::pick_unused_port().expect("No ports availabe");
    let listener = TcpListener::bind(("localhost", port))?;
    
    println!("Listening on \"localhost:{}\"", port);

    test_connection(("localhost", port))?;  //  Just a test

    for stream in listener.incoming() {

        match  stream {
            Ok(stream) => {
                println!("\n-----new stream!-----");
                handle_client(stream);
            },
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}
