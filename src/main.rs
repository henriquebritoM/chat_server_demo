
use std::{path::Path, str::FromStr};
use std::env;
use crate::{client::Client, server::Server};

mod message;
mod client;
mod server;
mod socket_json_utils;
mod channel_manager;

enum InitMode {
    Server,
    Client,
    Default
}

struct InvalidString;
impl FromStr for InitMode {
    type Err = InvalidString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InitMode::*;
        match s.to_lowercase().as_str() {
            "client" => return Ok(Client),
            "server" => return Ok(Server),
            "default" => return Ok(Default),
            _ => return Err(InvalidString),
        }
    }
}

///  Recebe o segundo argumento passado e o converte
///  para o enum InitMode
///  InitMode::Default é usado caso não sejam passados argumentos 
///  Retorna None se o argumento não puder ser convertido
fn get_mode() -> Option<InitMode> {
    
    let arg: String = env::args().nth(1).unwrap_or("default".to_string());
    
    return InitMode::from_str(&arg).ok();
}

/// Inicializa o programa de acordo com o modo passado
fn init_on_mode(mode: InitMode, path: &Path) -> std::io::Result<()> {

    let test_client: Option<Client> = Server::try_connection(path);
    let server_on: bool = test_client.is_some(); 

    match mode {
        InitMode::Server => {
            if server_on {
                println!("Já há um servidor online");
            }
            else {
                Server::run(path)?;
            }
        },
        InitMode::Client => {
            if server_on {
                Client::run(test_client.unwrap());
            }
            else {
                println!("Não há nenhum servidor ativo!")
            }            
        },
        InitMode::Default => {
            if server_on {
                Client::run(test_client.unwrap());
            }
            else {
                Server::run(path)?;
            }
        },
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    
    println!("Starting");

    let mode: InitMode = get_mode().expect("Selecione o modo [server|client|default]");
    let json_path = Path::new("socket.json");

    init_on_mode(mode, json_path)?;  

    Ok(())
}
