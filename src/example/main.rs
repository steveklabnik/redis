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

fn send_command_to_redis(mut redis: net::tcp::TcpStream, command: &[u8]) -> ~str{
    redis.write(command);

    let buf: &mut [u8] = [0];
    let mut response : ~[u8] = ~[];

    while(buf[0] != 13) {
        match redis.read(buf) {
            Some(_i) => {
                response.push(buf[0]);
            }
            None => println("Couldn't read off of the socket.")
        }
    }

    str::from_utf8(response)
}


fn main() {
    let ip = Ipv4Addr(127, 0, 0, 1);
    let socket = net::tcp::TcpStream::connect(SocketAddr { ip: ip, port: 6379 });
    match socket {
        Some(redis) => {
            let response = send_command_to_redis(redis, bytes!("*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$7\r\nmyvalue\r\n"));
            println(response);
        },
        None => println("Couldn't connect to a socket")
    }
}
