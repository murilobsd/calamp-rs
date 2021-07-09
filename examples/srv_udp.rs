use calamp_rs::Message;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    loop {
        let mut buf = [0u8; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("Receive: {} bytes from: {:?}", amt, &src);
        let msg = Message::parse(&buf);

        let msg_type: String = format!("{}", msg.message_header.message_type);
        let buf = msg_type.as_bytes();
        socket.send_to(buf, &src)?;
    }
}
