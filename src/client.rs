use std::{io::{BufRead, BufReader, Write}, net::{SocketAddr, TcpStream}, path::Path};

use crate::socket_json_utils::get_addr_from_json;

/// Struct Client
/// contém as funções e métodos para gerenciar toda a parte do do lado client
pub struct Client {
    stream: TcpStream,
}

impl Client {

    /// Cuida de todo a lógica de um novo cliente.   <br>
    /// Aborta com early return se não for possível criar o Client ou o servidor se desconectar     <br>
    /// <br>
    /// Recebe inputs do stdin, as envia para a self.stream e depois lê a resposta na self.stream
    /// 
    /// # panics
    /// - Se houver algum problema com o stdin/stdout
    pub fn run(mut client: Client) {
        
        let mut message: String = String::new();
        let mut _response: String = String::new();

        loop {
            //  leitura das inputs
            print!("Mensagem: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin()
            .read_line(&mut message)
            .expect("Failed to read input"); 
            
            //  envio das inputs para a self.TcpStream
            message.push_str("\r\n\r\n");    
            let m_sent = client.send_message(message.as_str());     //  Retorna erro se o servidor se desconectar

            if m_sent.is_err() {
                println!("Servidor se desconectou!");
                break;
            }

            //  leitura da resposta da self.TcpStream
            _response = client.read_response();
            println!("Recebido: {}", _response.trim());

            message.clear();
        }

    }

    /// Cria um nova instância de Client <br>
    /// Retorna None se:
    /// - O arquivo do path estiver vazio ou não existir, nesse caso, também cria o arquivo-destinho <br>
    /// - Houver algum erro conectando ao SocketAddr 
    pub fn new(path: &Path) -> Option<Client> {
        let addr = Client::get_addr(path)?;
        //  Esse erro aparece quando não há um servidor escutando a porta   <br>
        //  transformar isso em um teste para ver se o servidor está online !
        let stream = TcpStream::connect(addr);

        match stream {
            Ok(strm) => return Some(Client { stream: strm } ),
            Err(_) => return None,
        }
    }

    /// Envia a message passada para a self.stream  <br>
    /// Retorna um std::io::Error se houver algum enquando envia a message, geralmente
    /// porque o TcpListener se desconectou.
    pub fn send_message(&mut self, message: &str) -> Result<(), std::io::Error> {
        let result_write = self.stream.write_all(message.as_bytes());
        
        match result_write {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    /// Retorna a String lida na self.stream. <br>
    /// Blocking - Essa função espera até algo ser enviado pela stream
    /// 
    /// # panics
    /// - Se houver algum erro lendo a stream
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

    /// Retorna um SocketAddr do arquivo json passado
    /// Retona None se o arquivo estiver vazio
    fn get_addr(path: &Path) -> Option<SocketAddr> {
        get_addr_from_json(path)
    }
}

