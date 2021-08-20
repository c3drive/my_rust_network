use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    // let server_socket = UdpSocket::bind(address)?;
    let server_socket = UdpSocket::bind(address).expect("Failed to bind socket");
    loop{
        let mut buf = [0u8; 1024];

        match server_socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("handling data from {}", src);
                print!("{}", str::from_utf8(&buf[..size]).expect("Failed to read"));
                server_socket.send_to(&buf[..size], src).expect("Failed to send response");
            },
            Err(e) => {
                eprintln!("could not recieve a datagram: {}", e);
            }
        }
    }

}

