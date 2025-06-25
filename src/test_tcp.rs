use std::{io::{Read, Write}, net::{TcpListener, TcpStream, ToSocketAddrs}};

pub fn test_connection  <A: ToSocketAddrs>(addr: A) -> std::io::Result<()> {

    let mut stream = TcpStream::connect(&addr)?;
    let mut listener = TcpListener::bind(&addr)?;

    let mut response = String::new();

    // stream.write_all(b"Hello, from the other side of the TCP!")?;
    stream.write_all(b"GET / HTTP/1.1")?;

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => _ = stream.read_to_string(&mut response),
    //         Err(e) => eprint!("{}", e),
    //     }
    // }

    // println!("Response from server {response}");

    Ok(())
}