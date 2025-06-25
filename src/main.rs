
use std::{io::{self, BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}, os::windows::process, path::Path};

use crate::test_tcp::test_connection;

mod test_tcp;

fn handle_client(mut stream: TcpStream) {
    let mut client: Client = Client::new(stream);
    let mut method: String = String::new();
    let mut path: String = String::new();
    let mut protocol: String = String::new();

    client.get_request();

    println!("will get info");

    (method, path, protocol) = client.get_info();

    println!("info gotten");
    println!("method: '{}', path: '{}', protocol: '{}'", &method, &path, &protocol);

    if method != "GET" || protocol != "HTTP/1.1" {return;}  //  we only accept GET requests with HTTP/1.1 as protocol.

    println!("valid method and protocol");

    match path.as_str() {
        "/" => client.handle_ok(),
        "echo" => client.handle_echo(&path),
        _ => client.handle_not_found(),
    }

    client.write_response();
}

struct Request {
    stream: TcpStream,
    method: String,
    path: String,
    protocol: String,
}

impl Request {
    fn new(mut stream: TcpStream) -> std::io::Result<Request> {
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
            
            if str?.is_empty() {break;}
            request.push_str("{&str?}\r\n\r\n");
        }
    
        println!("Request: {}", request);
    
        Ok(request)
    }

    fn handle(&mut self) -> std::io::Result<()> {
        let mut target: &str = "";
        let mut content: &str = "";
        let mut response: String = String::new();
        
        match self.path.rsplit_once("/") {
            None => {},
            Some((str1, str2)) => {
                content = str1;
                target = str2;
            }
        }

        match target {
            "" => response = "HTTP/1.1 200 OK\r\n\r\n".to_string(),
            "echo" => response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-length: {}\r\n\r\n{}", content.len(), content).to_string(),
            _ => response = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
        }

        self.stream.write_all(response.as_bytes())?;

        Ok(())
    }
    
}

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

    fn get_request(&mut self) {

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

    //  Returns Method, Path, Protocol
    fn get_info(&self) -> (String, String, String) {
        let slices: Vec<&str> = self.request[0].split(' ').collect();
        //  Get / abc/abc/abc HTTP/1.1
        //   0  1      2
        return (slices[0].to_owned(), slices[1].to_owned(), slices[2].to_owned());
    }

    fn handle_ok(&mut self) {
        self.response.push_str("HTTP/1.1 200 OK\r\n\r\n");
    }

    fn handle_echo(&mut self, path: &str) {
        //let content
    }

    fn handle_not_found(&mut self) {
        self.response.push_str("HTTP/1.1 404 NOT FOUND\r\n\r\n");
    }

    fn get_response(&mut self) {

        match self.request[0].as_str() {
            "GET / HTTP/1.1" => self.response.push_str("HTTP/1.1 200 OK\r\n\r\n"),
            "GET /echo/abc HTTP/1.1" => {

            }
            _ => self.response.push_str("HTTP/1.1 404 NOT FOUND\r\n\r\n"),
        }

        println!("{}", self.response);
    }

    //  takes ownership of the client to drop it after writing to stream
    fn write_response(mut self) {
        _ = self.stream.write_all(self.response.as_bytes());
        _ = self.stream.shutdown(std::net::Shutdown::Both);
    }
}

fn main() -> std::io::Result<()> {
    
    println!("Hello, world!");

    let listener = TcpListener::bind("localhost:9090")?;
    let mut client: Client; 

    test_connection("localhost:9090")?;  //  Just a test

    for stream in listener.incoming() {

        match  stream {
            Ok(stream) => {
                println!("new stream!");
                handle_client(stream);
                // handle_client(stream);
                // client = Client::new(stream);
                // client.get_request();
                // client.get_response();
                // client.write_response();
            },
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}
