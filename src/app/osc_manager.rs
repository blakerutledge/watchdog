use rosc::{encoder, OscMessage, OscPacket, OscType};
// use std::f32;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
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

    let sock = Arc::new(Mutex::new(socket_host));

    println!("OSC Listening to {}", address_host);

    // let (tx, rx) = mpsc::channel();

    // Heartbeat Sending thread
    let send_sock = Arc::clone(&sock);
    thread::spawn(move || loop {
        {
            println!("1: lock plz");
            let locked_sock = send_sock.lock().unwrap();
            println!("1: lock thx");
            send_heartbeat(&locked_sock, &address_client);
            println!("sending heartbeat");
            std::mem::drop(locked_sock);
            println!("1: drop");
        }
        thread::sleep(Duration::from_secs(2));
    });

    // Heartbeat Receiving thread
    let recv_sock = Arc::clone(&sock);
    thread::spawn(move || {
        let mut buffer = [0u8; rosc::decoder::MTU];
        loop {
            println!("2: lock plz");
            let locked_sock = recv_sock.lock().unwrap();
            println!("2: lock thx");
            let empty = (0 as usize, address_empty);
            let msg: (usize, SocketAddr) = (locked_sock.recv_from(&mut buffer).unwrap_or(empty));
            if msg.0 > 0 {
                println!("Received packet with size {} from: {}", msg.0, msg.1);
                let (_, packet) = rosc::decoder::decode_udp(&buffer[..msg.0]).unwrap();
                handle_packet(packet);
            }
            std::mem::drop(locked_sock);
            println!("2: drop");
        }
        // thread::sleep(Duration::from_millis(50));
    });

    // send_thread.join().unwrap();
    // recv_thread.join().unwrap();
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            println!("OSC arguments: {:?}", msg.args);
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);
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

fn receive_heartbeat() {}
