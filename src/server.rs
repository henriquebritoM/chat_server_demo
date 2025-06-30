use std::{io::{self, BufRead, BufReader, Error, Write}, net::{SocketAddr, TcpListener, TcpStream}, path::Path};

use crate::{client::Client, socket_json_utils::send_addr_to_json};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn run(json_path: &Path) -> io::Result<()> {

        let server = Server::new()?;
        let addr = server.get_addr();

        send_addr_to_json(json_path, addr);

        for stream in server.listener.incoming() {

            if stream.is_err() {
                continue;   /* Ignore clients that could not connect */
            }

            std::thread::spawn(move || Server::handle_client(stream.unwrap()) );
        }

        Ok(())
    }

    pub fn new() -> Result<Server, Error> {

        let port = portpicker::pick_unused_port().expect("No ports availabe");
        let listener: TcpListener = TcpListener::bind(("localhost", port))?;
        
        println!("\nServidor online");
        println!("Listening on \"localhost:{}\"", port);

        return Ok( Server { listener } );
    }

    pub fn get_addr(&self) -> SocketAddr {
    
        return self.listener.local_addr().unwrap();
    }

    // Echos client message
    //  Not a method, it is intended to be called on a separete thread
    //  A method would move the self
    fn handle_client(mut stream: TcpStream) {

        println!("New client connected! addr: {}", stream.peer_addr().unwrap());

        loop {
            let message_res = Server::read_stream(&stream);
    
            if message_res.is_err() {
                return; 
                /*  Failed to read from client, aborting */
            }

            let mut message = message_res.unwrap();
            message.push_str("\r\n\r\n");
    
            if stream.write_all(message.as_bytes()).is_err() {
                return;
                /* Do nothing if could not write to client */
            };
            
        }
    }

    fn read_stream(stream: &TcpStream) -> Result<String, ()> {
        
        let mut request: String = String::new();
        let buf_reader = BufReader::new(stream);
        
        for str_result in buf_reader.lines() {
            if str_result.is_err() {
                break;
            }
            let str = str_result.unwrap();
            if str.is_empty() {
                break;
            }
            request.push_str(&str);
        }

        if request.is_empty() {
            return Err(());
        }

        println!("Read from client: {:#?}", request);
    
        Ok(request)
    }

    pub fn is_online(json_path: &Path) -> bool{
        
        let client_opt = Client::new(json_path);
        match client_opt {
            Some(_) => return true,
            None => return false,
        }
    }
    
}