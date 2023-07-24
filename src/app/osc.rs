use rosc::{encoder, OscMessage, OscPacket, OscType};

// use super::utils::now;

use super::config;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::time::Duration;

// const LOCAL_IP: String = ;

#[derive(Debug)]
pub struct Interface {
    socket_send: UdpSocket,
    socket_recv: UdpSocket,
    client: SocketAddrV4,
    empty: SocketAddr,
    channel: String,
}

impl Interface {
    pub fn build(c: &config::WatchedApp) -> Option<Interface> {
        // Convert ports to u16
        let port_host: u16 = u16::from(&c.osc_out_port.val);
        let port_client: u16 = u16::from(&c.osc_in_port.val);

        // Create IPV4 validated type for local host
        let host_str = "127.0.0.1";
        let host = match Ipv4Addr::from_str(host_str) {
            Ok(host) => host,
            Err(_) => panic!("Could not create IPV4 from address: {}", host_str),
        };

        // Create IPV4 addresses for both ports
        let address_host = SocketAddrV4::new(host, port_host);
        let address_client = SocketAddrV4::new(host, port_client);

        // Bind to the port
        let host_err_str = format!("OSC failed to bind to {:?}", address_host);
        let socket_host = UdpSocket::bind(address_host).expect(&host_err_str);

        let _ = socket_host.set_read_timeout(Some(Duration::from_millis(10)));

        let empty = SocketAddr::from_str("127.0.0.1:9999").unwrap();
        //
        // TODO: handle the error without panicking
        //

        let socket_host_clone = socket_host.try_clone().unwrap();

        let i = Interface {
            socket_recv: socket_host,
            socket_send: socket_host_clone,
            client: address_client,
            channel: String::from(&c.heartbeat_channel.val),
            empty: empty,
        };

        Some(i)
    }

    pub fn listen(&self, buffer: &mut [u8; rosc::decoder::MTU]) -> bool {
        // Retrieve message from the socket
        // let msg: usize = self.socket_recv.recv_from(buffer).unwrap_or_else(|_| []);
        let empty = (0 as usize, self.empty);
        let msg: (usize, SocketAddr) = self.socket_recv.recv_from(buffer).unwrap_or(empty);
        if msg.0 > 0 {
            let (_, packet) = rosc::decoder::decode_udp(&buffer[..msg.0]).unwrap();
            let is_heartbeat = self.receive_heartbeat(packet);
            return is_heartbeat;
        } else {
            return false;
        }
    }

    // Handler for incoming OSC messages, parse if it is a valid heartbeat
    fn receive_heartbeat(&self, packet: OscPacket) -> bool {
        let mut is_heartbeat = false;
        match packet {
            OscPacket::Message(msg) => {
                if msg.addr == self.channel {
                    is_heartbeat = true;
                } else {
                    self.receive_unexpected(msg);
                    is_heartbeat = false;
                };
            }
            OscPacket::Bundle(bundle) => {
                for p in bundle.content {
                    match p {
                        OscPacket::Message(msg) => {
                            if msg.addr == self.channel {
                                is_heartbeat = true;
                            } else {
                                self.receive_unexpected(msg);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        is_heartbeat
    }

    pub fn send_heartbeat(&self) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: self.channel.clone(),
            args: vec![OscType::Int(1)],
        }))
        .unwrap();

        self.socket_send.send_to(&msg_buf, &self.client).unwrap();
    }

    fn receive_unexpected(&self, msg: OscMessage) {
        println!("Unhandled OSC address: {}", msg.addr);
        println!("Unhandled OSC arguments: {:?}", msg.args);
    }
}
/*
pub struct Port {
    value: usize,
    valid: bool,
}

impl Port {
    fn new(p: usize) -> Self {
        let mut me = Port {
            value: p,
            valid: false,
        };

        me.validate();

        me
    }

    fn validate(&mut self) {
        self.valid = 1024 <= self.value && self.value <= 9999;
    }
}

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

*/
