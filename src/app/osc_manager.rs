use rosc::{encoder, OscMessage, OscPacket, OscType};

// use super::utils::now;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

struct OscInterface {
    host: UdpSocket,
    client: SocketAddrV4,
}

pub fn init() {
    // Local Host
    let addr_str = String::from("127.0.0.1");
    let port_host: u16 = 1234;
    let port_client: u16 = 1235;

    let host = match Ipv4Addr::from_str(&addr_str) {
        Ok(host) => host,
        Err(_) => panic!("{}", addr_str),
    };

    let address_host = SocketAddrV4::new(host, port_host);
    let address_client = SocketAddrV4::new(host, port_client);
    let address_empty = SocketAddr::from_str("127.0.0.1:9999").unwrap();

    let host_err_str = format!("OSC failed to bind to {:?}", address_host);
    let socket_host = UdpSocket::bind(address_host).expect(&host_err_str);

    let socket_send = socket_host.try_clone().unwrap();

    println!("OSC Listening to {}", address_host);

    // Heartbeat Sending thread
    thread::spawn(move || loop {
        {
            send_heartbeat(&socket_send, &address_client);
            println!("Sending heartbeat...");
        }
        thread::sleep(Duration::from_secs(2));
    });

    // Heartbeat Receiving thread
    thread::spawn(move || {
        let mut buffer = [0u8; rosc::decoder::MTU];
        loop {
            let empty = (0 as usize, address_empty);
            let msg: (usize, SocketAddr) = (socket_host.recv_from(&mut buffer).unwrap_or(empty));
            if msg.0 > 0 {
                let (_, packet) = rosc::decoder::decode_udp(&buffer[..msg.0]).unwrap();
                handle_packet(packet);
            }
        }
    });
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            if msg.addr == "/heart" {
                receive_heartbeat();
            } else {
                receive_unexpected(msg);
            }
        }
        OscPacket::Bundle(bundle) => {
            for p in bundle.content {
                match p {
                    OscPacket::Message(msg) => {
                        if msg.addr == "/heart" {
                            receive_heartbeat();
                        } else {
                            receive_unexpected(msg);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn send_message(socket: &UdpSocket, to_addr: &SocketAddrV4, channel: String) {
    let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: channel,
        args: vec![OscType::Int(1)],
    }))
    .unwrap();

    socket.send_to(&msg_buf, to_addr).unwrap();
}

fn send_heartbeat(socket: &UdpSocket, to_addr: &SocketAddrV4) {
    send_message(socket, to_addr, "/heart".to_string());
}

fn receive_heartbeat() {
    println!("Receive heartbeat!");
    println!("- - - - ");
}

fn receive_unexpected(msg: OscMessage) {
    println!("Unhandled OSC address: {}", msg.addr);
    println!("Unhandled OSC arguments: {:?}", msg.args);
}
