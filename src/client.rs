use std::{io::{BufRead, BufReader, Write}, net::{SocketAddr, TcpStream}, path::Path, sync::mpsc::{channel, Sender}};
use crate::socket_json_utils::get_addr_from_json;
use crate::channel_manager::ClientChannelManager;

/// Struct Client
/// contém os channels para comunicação com as threads e o handler para a stream
pub struct Client {
    stream_handler: StreamHandler,
    channels: ClientChannelManager,
}

impl Client {

    /// Cria um nova instância de Client <br>
    /// Retorna None se não for possível se conectar ao servidor
    pub fn new(path: &Path) -> Option<Client> {

        //  None se não houver nada no socket.json (ou algo inválido)
        let addr = Client::get_addr(path)?;
        //  None quando não há um servidor escutando a porta
        let stream = TcpStream::connect(addr).ok()?;

        stream.set_nonblocking(true).unwrap();
        let (tx, rx) = channel::<String>();
        let channel_manager = ClientChannelManager::new(tx, rx);
        let stream_handler = StreamHandler{ stream: stream };

        return Some(Client { stream_handler: stream_handler, channels: channel_manager } )
            
    }

    /// Cuida de todo a lógica de um novo cliente.   <br>
    /// <br>
    /// Monitora o stdin e a stream, envia as entradas io para a stream e da println no recebido pela stream
    pub fn run(mut client: Client) {

        println!("Conectado !");

        //  Cria a thread que monitora o stdin
        //  ela pode dar panic caso ocorra algum erro, 
        //  parando de enviar mensagens para o channel
        //  todo! desconectar client se stdin der problemas
        let temp_sender = client.channels.sender.clone();
        std::thread::spawn(|| IoHandler::read_io(&IoHandler { sender: temp_sender }));
        
        let mut stream_message: Option<String>;
        let mut io_input: Option<String>;
        loop {

            io_input = client.channels.receive();
            stream_message = client.stream_handler.read_stream();

            //  Envia mensagens do IO para a stream
            match io_input {
                None => {},
                Some(strg) => {
                    let res = client.stream_handler.write_message(&strg);
                    if res.is_err() {
                        //  Servidor se desconectou, break do main loop
                        break;
                    }
                },
            }

            //  Println das mensagens recebidas da stream
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

/// Struct StreamHandler
/// Implementa as operações básicas para interagir com a stream
struct StreamHandler {
    stream: TcpStream,
}

impl StreamHandler {

    /// Recebe mensagens pela stream
    /// Clients criados com Client::new() já vem com 
    /// set_nonblocking(true
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

/// Struct IoHandler
/// Feito para ser usado em uma thread separada. <br>
/// Recebe inputs do stdin e envia para o channel
struct IoHandler {
    sender: Sender<String>,
}

impl IoHandler {

    /// blocking
    /// Recebe input do stdin e envia para o channel <br>
    /// # panic <br>
    /// - Se houver algum erro no stdin/stdout <br>
    /// - Se não for possível enviar a mensagem pelo channel <br>
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

