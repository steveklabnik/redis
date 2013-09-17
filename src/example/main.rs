extern mod redis;

use std::rt::io::*;
use std::rt::io::net::ip::*;
use std::io::println;

use std::str;

fn main() {
    let ip = Ipv4Addr(127, 0, 0, 1);
    let mut socket = net::tcp::TcpStream::connect(SocketAddr { ip: ip, port: 6379 });
    socket.write(bytes!("*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$7\r\nmyvalue\r\n"));
    //let bytes = socket.read_to_end();
    //println(std::str::from_utf8(bytes));
    let mut buf: &mut [u8] = [0, 0, 0, 0];
    match socket.read(buf) {
        Some(uint) => println(str::from_utf8(buf)),
        None => println("NOPE")
    }
}
