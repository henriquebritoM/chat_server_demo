use std::{net::SocketAddr, path::Path};
use serde_json::from_str;

pub fn get_addr_from_json(path: &Path) -> SocketAddr {

    check_file(path).unwrap();

    let data: String = std::fs::read_to_string(path).unwrap();

    if data.is_empty() {
        panic!("Client found no addres in {:?}", path);
    }

    return from_str::<SocketAddr>(&data).unwrap()
}

pub fn send_addr_to_json(path: &Path, addr: SocketAddr) {

    check_file(path).unwrap();

    let socket_serialized: String = serde_json::to_string_pretty(&addr).unwrap();

    std::fs::write(path, socket_serialized).unwrap();
}

fn check_file(path: &Path) -> std::io::Result<()> {

    if !path.exists() {
        println!("No {:?} found, attempting to create file...", path);
        std::fs::File::create(path).expect("could not create file");
        println!("file created!");
    }

    Ok(())
}

