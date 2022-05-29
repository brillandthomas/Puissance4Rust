use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:50001").unwrap();
    let mut in_buffer = [0; 1024];
    let mut out_buffer = String::new();
    loop {
        io::stdin().read_line(&mut out_buffer).unwrap();
        stream.write(out_buffer.as_bytes()).unwrap();
        out_buffer.clear();
        stream.read(&mut in_buffer).unwrap();
        let message = String::from_utf8_lossy(&in_buffer[..]);
        println!("{}", message);
        in_buffer.iter_mut().for_each(|m| *m = 0);
    }
}
