use std::sync::mpsc::{Receiver, Sender};

use crate::message::Message;

/// Channel manager para o servidor <br>
/// envia mensagem para todas as threads <br>
/// recebe mensagem de todas as threads <br>
pub struct ServerChannelManager {
    senders_a: Vec< Sender<Message>>,       //  Vec de channels main -> thread (a)
    receiver_b: Receiver<Message>,          //  Receviver para thread -> main (b)
    pub sender_b: Sender<Message>,          //  sender para thread -> main (b)      [para clonar]
}

impl ServerChannelManager {
    
    /// Cria uma novo ServerChannelManager <br>
    pub fn new(sender_b: Sender<Message>, receiver_b: Receiver<Message>) -> ServerChannelManager {
        return ServerChannelManager {
            senders_a: Vec::new(), 
            sender_b: sender_b,
            receiver_b: receiver_b,
        };
    }

    /// Adiciona um novo sender ao vetor 
    pub fn add_sender(&mut self, sender_a: Sender<Message>) {
        self.senders_a.push(sender_a);
    }   

    /// Remove um sender do vetor 
    fn remove_sender(&mut self, index: usize) {
        self.senders_a.swap_remove(index);
    }

    /// Remove varios senders do vetor
    /// Remove os indexes em ordem decrescente para não causar problemas durante a iteração
    fn remove_senders(&mut self, mut senders: Vec<usize>) {

        senders.sort();
        senders.reverse();    
        //  Como erros está em ordem decrescente, não devem haver conflitos
        for sender in senders {
            self.senders_a.remove(sender);
        }
    }

    /// non-blocking
    /// Tenta receber uma mensagem das threads
    pub fn receive_message(&mut self) -> Option<Message> {

        match self.receiver_b.try_recv() {
            Ok(msg) => return Some(msg),
            Err(_) => return None,
        }

    }

    /// Envia uma mensagem para todas as threads
    /// Remove as threads que se desconectaram
    pub fn send_message(&mut self, message: Message) {
        let mut erros: Vec<usize> = Vec::new(); //  index dos senders que falharam em enviar mensagem, receiver se desconectou

        for i in 0..self.senders_a.len() {
            match self.senders_a[i].send(message.clone()) {
                Ok(_) => {},
                Err(_) => {
                    erros.push(i);
                    //println!("erro: {}", e);
                },
            }
        } 

        //println!("erros len {}", erros.len());

        if erros.is_empty() {
            return;
        };

        //  Exclui os senders que já se desconectaram
        self.remove_senders(erros);

    }
}

/// Channel manager para o client <br>
/// recebe mensagens pelo receiver
/// acesso ao sender para clonar
pub struct ClientChannelManager {
    pub sender: Sender<String>,
    receiver: Receiver<String>,
}

impl ClientChannelManager {
    pub fn new(sender: Sender<String>, receiver:Receiver<String> ) -> ClientChannelManager {
        return ClientChannelManager {sender, receiver};
    } 

    pub fn receive(&mut self) -> Option<String> {
        match self.receiver.try_recv() {
            Ok(msg) => return Some(msg),
            Err(_) => return None,
        }
    }

}
