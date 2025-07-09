use std::{io::{BufRead, BufReader, Write}, net::{SocketAddr, TcpStream}, path::Path, sync::mpsc::{channel, Sender}};
use crate::socket_json_utils::get_addr_from_json;
use crate::channel_manager::ClientChannelManager;

/// Struct Client
/// contém as funções e métodos para gerenciar toda a parte do do lado client
pub struct Client {
    stream_handler: StreamHandler,
    channels: ClientChannelManager,
}

impl Client {

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
            Err(_) => return None,
            Ok(strm) => {
                strm.set_nonblocking(true).unwrap();
                let (tx, rx) = channel::<String>();
                let channel_manager = ClientChannelManager::new(tx, rx);
                let stream_handler = StreamHandler{ stream: strm };
                return Some(Client { stream_handler: stream_handler, channels: channel_manager } )
            
            },
        }
    }

    /// Cuida de todo a lógica de um novo cliente.   <br>
    /// Aborta com early return se não for possível criar o Client ou o servidor se desconectar     <br>
    /// <br>
    /// Recebe inputs do stdin, as envia para a self.stream e depois lê a resposta na self.stream
    /// 
    /// # panics
    /// - Se houver algum problema com o stdin/stdout
    pub fn run(mut client: Client) {

        println!("Conectado !");

        let temp_sender = client.channels.sender.clone();
        std::thread::spawn(|| IoHandler::read_io(&IoHandler { sender: temp_sender }));
        
        let mut stream_message: Option<String>;
        let mut io_input: Option<String>;
        loop {

            io_input = client.channels.receive();
            stream_message = client.stream_handler.read_stream();

            match io_input {
                None => {},
                Some(strg) => {
                    let res = client.stream_handler.write_message(&strg);
                    if res.is_err() {
                        break;
                    }
                },
            }

            match stream_message {
                None => {},
                Some(strg) => println!("<-{}", strg),
            }
        }

        println!("servidor se desconectou");

    }

    /// Retorna um SocketAddr do arquivo json passado
    /// Retona None se o arquivo estiver vazio
    fn get_addr(path: &Path) -> Option<SocketAddr> {
        get_addr_from_json(path)
    }
}

struct StreamHandler {
    stream: TcpStream,
}

impl StreamHandler {

    /// Non blocking if stream is set to non-blocking
    fn read_stream(&self) -> Option<String> {
        
        let mut request: String = String::new();
        
        let buf_reader = BufReader::new(&self.stream);
        for str_result in buf_reader.lines() {
            
            if str_result.is_err() {
                break;  /* Força parada da leitura */
            }

            let str = str_result.unwrap();
            if str.is_empty() {
                break;  /* Fim da stream, para de ler */
            }
            request.push_str(&str);
        }
        
        if request.is_empty() {
            return None;
        }

        Some(request)
    }

    /// Envia a message passada para a self.stream  <br>
    /// Retorna um std::io::Error se houver algum enquando envia a message, geralmente
    /// porque o TcpListener se desconectou.
    pub fn write_message(&mut self, message: &str) -> Result<(), std::io::Error> {
        if message.is_empty() {
            return Ok(());
        }
        let result_write = self.stream.write_all(message.as_bytes());

        match result_write {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    
}

struct IoHandler {
    sender: Sender<String>,
}

impl IoHandler {

    /// blocking
    fn read_io(&self) {

        loop {

            let mut message: String = String::new();
            //  leitura das inputs
            print!("->");
            std::io::stdout().flush().unwrap();
            std::io::stdin()
            .read_line(&mut message)
            .expect("Failed to read input"); 
    
            self.sender.send(message).unwrap();
        }

    }
}

