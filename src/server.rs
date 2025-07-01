use std::{io::{self, BufRead, BufReader, Error, Write}, net::{SocketAddr, TcpListener, TcpStream}, path::Path, sync::mpsc::{Receiver, Sender}};

use crate::{client::Client, socket_json_utils::send_addr_to_json};

pub struct Server {
    listener: TcpListener,
    mpsc_receiver: Receiver<String>,
    mpsc_sender: Sender<String>
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

            let mut handler = ServerHandler::new(stream.unwrap(), server.mpsc_sender.clone());
            std::thread::spawn(move || handler.handle_client() );
        }

        Ok(())
    }

    pub fn new() -> Result<Server, Error> {

        let port = portpicker::pick_unused_port().expect("No ports availabe");
        let listener: TcpListener = TcpListener::bind(("localhost", port))?;
        let (mpsc_sender, mpsc_receiver) = std::sync::mpsc::channel::<String>();
        
        println!("\nServidor online");
        println!("Listening on \"localhost:{}\"", port);

        return Ok( Server { listener, mpsc_receiver, mpsc_sender } );
    }

    
    
    pub fn try_connection(json_path: &Path) -> Option<Client> {
        
        let client_opt = Client::new(json_path);
        match client_opt {
            Some(c) => return Some(c),
            None => return None,
        }
    }
    
    pub fn get_addr(&self) -> SocketAddr {
    
        return self.listener.local_addr().unwrap();
    }
}

struct ServerHandler {
    stream: TcpStream,
    mpsc_sender: Sender<String>
}

impl ServerHandler {

    fn new(stream: TcpStream, mpsc_sender: Sender<String>) -> ServerHandler {
        return ServerHandler { stream, mpsc_sender };
    }

    // Echos client message
    //  Not a method, it is intended to be called on a separete thread
    //  A method would move the self
    //  todo : move to another struct
    fn handle_client(&mut self) {
        
        println!("New client connected! addr: {}", self.stream.peer_addr().unwrap());
        
        loop {
            let message_res = self.read_stream();
            
            if message_res.is_err() {
                return; 
                /*  Failed to read from client, aborting */
            }
            
            let mut message = message_res.unwrap();
            message.push_str("\r\n\r\n");
            
            self.write_stream(&message);
        }
    }
    
    fn read_stream(&self) -> Result<String, ()> {
        
        let mut request: String = String::new();
        let buf_reader = BufReader::new(&self.stream);
        
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
    
    fn write_stream(&mut self, message: &str) {
        
        /* Do nothing if could not write to client */
        _ = self.stream.write_all(message.as_bytes());
    }
}