use std::net::TcpStream;

pub fn run(socket_address: (String, u16)) {
    let server = TcpStream::connect(socket_address).unwrap();
}
