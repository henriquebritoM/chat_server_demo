use std::{io::{self, BufRead, BufReader, Error, Write}, net::{SocketAddr, TcpListener, TcpStream}, path::Path};

use crate::socket_json_utils::send_addr_to_json;

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

            server.handle_client(stream.unwrap());
        }

        Ok(())
    }

    pub fn new() -> Result<Server, Error> {

        let port = portpicker::pick_unused_port().expect("No ports availabe");
        let listener: TcpListener = TcpListener::bind(("localhost", port))?;
        
        println!("Listening on \"localhost:{}\"", port);

        return Ok( Server { listener } );
    }

    pub fn get_addr(&self) -> SocketAddr {
    
        return self.listener.local_addr().unwrap();
    }


    // Echos client message
    fn handle_client(&self, mut stream: TcpStream) {

        println!("New client connected! addr: {}", stream.peer_addr().unwrap());

        loop {
            let message = self.read_stream(&stream);
    
            if message.is_err() {
                return; 
                /*  Failed to read from client, aborting */
            }
    
            if stream.write_all(message.unwrap().as_bytes()).is_err() {
                return;
                /* Do nothing if could not write to client */
            };
            
        }
    }

    fn read_stream(&self, stream: &TcpStream) -> io::Result<String> {
        
        let mut request: String = String::new();
        let buf_reader = BufReader::new(stream);
        
        for str in buf_reader.lines() {
            let str = str?;
            if str.is_empty() {break;}

            request.push_str(&format!("{}\r\n\r\n", str));
        }
    
        println!("Read from client: {:#?}", request);
    
        Ok(request)
    }
    
}