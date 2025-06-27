use std::{io::Write, net::{SocketAddr, TcpStream}, path::Path};
use crate::socket_json_utils::get_addr_from_json;

pub struct Client {
    stream: TcpStream,
    message: String,
}

impl Client {
    pub fn new(path: &Path, message: String) -> Client {

        let addr = Client::get_addr(path);
        let stream = TcpStream::connect(addr);
        
        if stream.is_err() {
            println!("ERRO CONECTANDO CLIENTE A STREAM {}", stream.is_err());
        }

        Client { stream: stream.unwrap(), message: message }
    }

    pub fn send_message(&mut self) {
        let result_write = self.stream.write_all(self.message.as_bytes());
        
        if result_write.is_err() {
            println!("Houve erro ao mandar mensagem? {:?} ", result_write.err().unwrap());
        }
    }

    fn get_addr(path: &Path) -> SocketAddr {
        get_addr_from_json(path)
    }
}