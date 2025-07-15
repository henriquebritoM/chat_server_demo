use std::{io::{self, BufRead, BufReader, Error, Write}, net::{SocketAddr, TcpListener, TcpStream}, path::Path, sync::mpsc::{channel, Receiver, SendError, Sender}};

use crate::{channel_manager::ServerChannelManager, client::Client, message::Message, socket_json_utils::send_addr_to_json};

pub struct Server {
    listener: TcpListener,
    channels: ServerChannelManager
}

impl Server {

    pub fn new() -> Result<Server, Error> {

        let port = portpicker::pick_unused_port().expect("No ports availabe");
        let listener: TcpListener = TcpListener::bind(("localhost", port))?;
        listener.set_nonblocking(true).expect("set nonbloking failed");
        let (my_sender, my_receiver) = channel::<Message>();

        let channel_manager = ServerChannelManager::new(my_sender, my_receiver);
        
        println!("\nServidor online");
        println!("Listening on \"localhost:{}\"", port);

        return Ok( Server { listener, channels: channel_manager } );
    }
    
    pub fn run(json_path: &Path) -> io::Result<()> {

        let mut server = Server::new()?;
        let addr = server.get_addr();
        let mut thread_id: u16 = 0;

        send_addr_to_json(json_path, addr);

        let mut stream_result:Result<(TcpStream, SocketAddr), Error>;
        let mut channel_message: Option<Message>;

        loop {
            stream_result = server.listener.accept();
            channel_message = server.channels.receive_message();

            match stream_result {
                Err(_) => {},
                Ok((stream, _)) => {
                    println!("Novo cliente");
                    let handler = server.set_up_thread(thread_id, stream);
                    std::thread::spawn(move || ThreadHandler::handle_client(handler));
                    thread_id += 1;
                },
            }

            match channel_message {
                None => {},
                Some(msg) => {
                    println!("Mensagem recebida no server");
                    server.channels.send_message(msg);
                },
            }

        }

        //  necessário implementar uma forma de shutdown
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

    fn set_up_thread(&mut self, id: u16, stream: TcpStream) -> ThreadHandler {

        let (sender_a, receiver_a) = channel::<Message>();

        self.channels.add_sender(sender_a.clone());         //  Sender main -> thread

        let handler = ThreadHandler::new(stream, self.channels.sender_b.clone(), receiver_a, id);
        return handler;
    }
}

struct ThreadHandler {
    stream: TcpStream,
    thread_sender: Sender<Message>,     // thread -> main (b)
    thread_receiver: Receiver<Message>, // main -> thread (a)
    id: u16
}

impl ThreadHandler {

    fn new(stream: TcpStream, thread_sender: Sender<Message>, thread_receiver: Receiver<Message>, id: u16) -> ThreadHandler {
        stream.set_nonblocking(true).expect("set_nonblocking failed");
        return ThreadHandler { stream, thread_sender, thread_receiver, id };
    }

    //  Not a method, it is intended to be called on a separete thread
    //  A method would move the self
    fn handle_client(mut handler: ThreadHandler) {

        // enum MessageFrom {
        //     Channel,
        //     Stream
        // }
        
        println!("New client connected! id: {}", handler.id);
        
        // let mut message_origin: MessageFrom;
        let mut stream_message: Option<Message>; //  A mensagem que um cliente manda pelo TCP. Deve ser enviada à outra struct do server
        let mut channel_message: Option<Message>;    //  A mensagem que o servidor manda. Deve ser enviada para a stream
        
        loop {
            stream_message = handler.read_stream();
            channel_message = handler.read_channel();

            match stream_message {
                None => {},
                Some(msg) => {
                    println!("{} Nova mensagem: {:?}", handler.id, msg);
                    handler.write_channel(msg).expect("main thread se desconectou");
                },
            }

            match channel_message {
                None => {},
                Some(msg) => {
                    println!("{} Nova mensagem da main", handler.id);
                    //  This message was send by this thread, ignore
                    println!("thread id: {}, msg id: {}", handler.id, msg.sender_id);
                    if handler.id == msg.sender_id {
                        continue;
                    }

                    handler.write_stream(msg);
                },
            }
        }

    }

    /// Non blocking
    /// Receives plain text from the client and creates a Message
    /// with it and the thread id
    fn read_stream(&self) -> Option<Message> {
        
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
        
        println!("{} Mensagem recebida: {}", self.id, request);
        Some(Message::new(request, self.id))
    }
    
    fn read_channel(&self) -> Option<Message> {
        match self.thread_receiver.try_recv() {
            Ok(mes) => return Some(mes),
            Err(_) => return None,
        }
    }

    //  None provavelmente indica que o servidor se desconectou
    //  talvez seja uma boa ideia para a execução do programa
    fn write_channel(&mut self, message: Message) -> Result<(), SendError<Message>> {  
        self.thread_sender.send(message)?;
        Ok(())
    }

    fn write_stream(&mut self, message: Message) {
        
        /* Do nothing if could not write to client */
        let tcp_text = message.text + "\r\n\r\n";
        println!("{} mandando mensagem para o cliente", self.id);
        self.stream.write_all(tcp_text.as_bytes()).unwrap();        //  Panic se o client tiver se desconectado todo()! clean exit
    }
}
