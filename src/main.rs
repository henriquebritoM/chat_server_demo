
use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use crate::test_tcp::connect_to_tcp;

mod test_tcp;

struct Client {
    stream: TcpStream,      //  The stream
    request: Vec<String>,   //  Each line of the request is stored as an element in this vec
    response: String        //  The Response
}

impl Client {

    fn new(stream: TcpStream) -> Client {
        return Client {
            stream: stream, 
            request: Vec::new(),
            response: String::new(),
        };
    }

    fn set_request(&mut self) {

        let buf_reader = BufReader::new(&self.stream);
        
        for str in buf_reader.lines() {
    
            match str {
                Ok(s) => {
                    if s.is_empty() {break;}
                    self.request.push(s);
                },
                Err(_) => {},
            }
            
        }

        println!("{:#?}", self.request);
    }

    fn set_response(&mut self) {

        match self.request[0].as_str() {
            "GET / HTTP/1.1" => self.response.push_str("HTTP/1.1 200 OK\r\n\r\n"),
            _ => self.response.push_str("HTTP/1.1 404 NOT FOUND\r\n\r\n"),
        }

        println!("{}", self.response);
    }

    fn write_response(mut self) {
        _ = self.stream.write_all(self.response.as_bytes());
        println!("not blocked");
    }
}

fn main() -> std::io::Result<()> {
    
    println!("Hello, world!");

    let listener = TcpListener::bind("localhost:9090")?;
    let mut client: Client;

    // connect_to_tcp("localhost:9090")?;  //  Just a test

    for stream in listener.incoming() {

        match  stream {
            Ok(stream) => {
                client = Client::new(stream);
                client.set_request();
                client.set_response();
                client.write_response();
            },
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}
