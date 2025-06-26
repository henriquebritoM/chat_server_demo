use std::{io::{Read, Write}, net::{TcpListener, TcpStream, ToSocketAddrs}};

pub fn test_connection  <A: ToSocketAddrs>(addr: A) -> std::io::Result<()> {

    let mut stream = TcpStream::connect(&addr)?;

    let mut response = String::new();

    stream.write(b"GET / HTTP/1.1")?;

    //  This does not send a new message, only appends to the existing one
    //  stream.write(b"Hello, from the other side of the TCP!")?;

    // println!("Response from server {response}");

    Ok(())
}