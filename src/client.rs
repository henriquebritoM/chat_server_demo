use std::{io::{BufRead, BufReader, Write}, net::{SocketAddr, TcpStream}, path::Path};
use crate::socket_json_utils::get_addr_from_json;

pub struct Client {
    stream: TcpStream,
    message: String,
}

impl Client {
    pub fn run(path:&Path) {
        let mut client = Client::new(path);

        loop {
            let mut message = String::new();
            let mut response = String::new();

            std::io::stdin()
            .read_line(&mut message)
            .expect("Failed to read input"); 

            client.set_message(message);
            client.stream.write_all(client.message.as_bytes()) .unwrap();

            response = client.read_response();
            println!("Response from server: {}", response);
        }
    }

    pub fn new(path: &Path) -> Client {
        let addr = Client::get_addr(path);
        let stream = TcpStream::connect(addr).expect("ERRO CONECTANDO CLIENTE A STREAM {}");

        Client { stream: stream, message: String::new() }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = format!("{}\r\n", message);
    }

    pub fn send_message(&mut self) {
        let result_write = self.stream.write_all(self.message.as_bytes());
        
        if result_write.is_err() {
            println!("Houve erro ao mandar mensagem? {:?} ", result_write.err().unwrap());
        }
    }

    pub fn read_response(&mut self) -> String {
        let mut response = String::new();

        let buf_reader = BufReader::new(&self.stream);
        
        for str_result in buf_reader.lines() {
            
            if str_result.is_err() {break;}

            let str = str_result.unwrap();
            if str.is_empty() {break;}

            response.push_str(&format!("{}\r\n\r\n", str));
        }

        response
    }

    fn get_addr(path: &Path) -> SocketAddr {
        get_addr_from_json(path)
    }
}