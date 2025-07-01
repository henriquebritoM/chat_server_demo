
use std::path::Path;
use std::env;
use crate::{client::Client, server::Server};

mod client;
mod server;
mod socket_json_utils;

//  Recebe o argumento que foi passado quando o programa foi chamado
//  É de se esperar dois argumentos:
//  O path de execução e o mode
//  Se forem passados mais ou menos parametros ocorreu um erro e retorna None
fn get_mode() -> Option<String> {
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        return None;
    }
    
    Some(args[1].to_owned())
}


fn main() -> std::io::Result<()> {
    
    println!("Starting");

    let mode = get_mode().expect("Selecione o modo [server|client]");
    let json_path = Path::new("socket.json");

    let test_client: Option<Client> = Server::try_connection(json_path);
    let server_on: bool = test_client.is_some();   

    match mode.as_str() {

        "server" => { 
            if server_on {
                println!("Já há um servidor online!");
            }
            else {
                Server::run(json_path)?;
            }
        },

        "client" => {
            if server_on {
                Client::run(test_client.unwrap());
            }
            else {
                println!("Não há nenhum servidor ativo!")
            }

        }
        _ => println!("Modo inválido, por favor digite 'server' ou 'client'"),
    }

    Ok(())
}
