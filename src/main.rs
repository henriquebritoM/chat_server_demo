
use std::{io::{BufRead, BufReader, Write}, net::TcpStream, path::Path};
use std::env;
use crate::{client::Client, server::Server, test_tcp::test_connection};

mod test_tcp;
mod client;
mod server;
mod socket_json_utils;

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

        //  Move this block to a dedicated method/fn
        match request.read_request()?.split_once("\r\n\r\n") {
            None => {},
            Some((str1, str2)) => {
                request_first_line.push_str(str1);
                request_rest.push_str(str2);
            }
        }
        
        //  Move this block to a dedicated method/fn
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
        
        //  Move this block to a dedicated method/fn
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

fn get_mode() -> Option<String> {
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        return None;
    }
    
    Some(args[1].to_owned())
}


fn main() -> std::io::Result<()> {
    
    println!("Starting");

    let mode = get_mode().expect("Selecione o modo [server|client]");
    let json_path = Path::new("socket.json");
    let mut server_on: bool = false;

    match mode.as_str() {

        "server" => {
            if !server_on {
                Server::run(json_path);
                server_on = true;
            }
            else {
                println!("Já há um servidor ativo!");
            }

        },

        "client" => {

        }
        _ => println!("Modo inválido, por favor digite 'server' ou 'client'"),
    }

    Ok(())
}
