extern mod redis;

use std::rt::io::*;
use std::rt::io::net::ip::*;
use std::io::println;

use std::str;

// TODO: These types are all wron, but they are all the types
enum Reply {
    Status(~str),
    Error(~str),
    Integer(int),
    Bulk(~str),
    MultiBulk(~str),
}


fn main() {
    let ip = Ipv4Addr(127, 0, 0, 1);
    let mut socket = net::tcp::TcpStream::connect(SocketAddr { ip: ip, port: 6379 });
    socket.write(bytes!("*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$7\r\nmyvalue\r\n"));
    let buf: & mut [u8] = [0];
    while(buf[0] != 13) {
        match socket.read(buf) {
            Some(i) => println(str::from_utf8(buf)),
            None => println("NOPE")
        }
    }
}
