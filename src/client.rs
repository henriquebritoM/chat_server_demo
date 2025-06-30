use std::{io::{BufRead, BufReader, Write}, net::{SocketAddr, TcpStream}, path::Path};

use crate::socket_json_utils::get_addr_from_json;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn run(path:&Path) {
        let client_opt = Client::new(path);
        let mut client: Client;
        
        let mut message: String = String::new();
        let mut _response: String = String::new();

        match client_opt {
            Some(c) => client = c,
            None => return,
        }

        loop {
            print!("Mensagem: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin()
            .read_line(&mut message)
            .expect("Failed to read input"); 
            
            message.push_str("\r\n\r\n");    
            let m_sent = client.send_message(message.as_str());     //  Retorna erro se o servidor se desconectar

            if m_sent.is_err() {
                println!("Servidor se desconectou!");
                break;
            }

            _response = client.read_response();
            println!("Recebido: {}", _response.trim());

            message.clear();
        }

    }

    pub fn new(path: &Path) -> Option<Client> {
        let addr = Client::get_addr(path);
        //  Esse erro aparece quando não há um servidor escutando a porta
        //  transformar isso em um teste para ver se o servidor está online !
        let stream = TcpStream::connect(addr);

        match stream {
            Ok(strm) => return Some(Client { stream: strm } ),
            Err(_) => return None,
        }
    }

    //  Escreve a mensagem passada para a stream
    //  Se falhar, provavelmente o servidor se desconectou
    pub fn send_message(&mut self, message: &str) -> Result<(), std::io::Error> {
        let result_write = self.stream.write_all(message.as_bytes());
        
        match result_write {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    pub fn read_response(&mut self) -> String {
        let mut response = String::new();

        let buf_reader = BufReader::new(&self.stream);
        
        for str_result in buf_reader.lines() {
            
            if str_result.is_err() {break;}

            let str = str_result.unwrap();
            if str.is_empty() {break;}

            response.push_str(&str);
        }

        response
    }

    fn get_addr(path: &Path) -> SocketAddr {
        get_addr_from_json(path)
    }
}

