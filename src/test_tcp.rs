use std::{io::Write, net::{TcpStream, ToSocketAddrs}};

pub fn connect_to_tcp  <A: ToSocketAddrs>(addr: A) -> std::io::Result<()> {

    let mut stream = TcpStream::connect(addr)?;

    stream.write_all(b"Hello, from the other side of the TCP!")?;

    Ok(())
}