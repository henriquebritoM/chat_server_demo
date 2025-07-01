use std::{net::SocketAddr, path::Path};
use serde_json::from_str;

/// Serializa o SocketAddr para o json passado, sobreescrevendo tudo no arquivo de destino. <br>
/// <br>
/// Se não houver nada no caminho passado, cria o arquivo e então serializa.
/// 
/// # panics
/// - Se não for possivel criar o arquivo
/// - Se não for possivel salvar o socket no arquivo 
pub fn send_addr_to_json(path: &Path, addr: SocketAddr) {

    check_file(path);

    let socket_serialized: String = serde_json::to_string_pretty(&addr).unwrap();

    std::fs::write(path, socket_serialized).unwrap();
}

/// Desserializa um SocketAddr do json passado e o retorna. <br>
/// Se não houver nada retorna none.
/// 
/// # panics
/// - Se não for possivel ler o arquivo
/// - Se não for possivel converter o conteúdo para SockeAddr
pub fn get_addr_from_json(path: &Path) -> Option<SocketAddr> {

    check_file(path);

    let data: String = std::fs::read_to_string(path).unwrap();  

    if data.is_empty() {
        return None;
    } 

    return Some(from_str::<SocketAddr>(&data).unwrap());
}

/// Cria um arquivo no path se o mesmo não existir.
/// 
/// # panics
/// - Se não for possível criar o arquivo
fn check_file(path: &Path) {

    if !path.exists() {
        println!("No {:?} found, attempting to create file...", path);
        std::fs::File::create(path).expect("could not create file");
        println!("file created!");
    }

}

