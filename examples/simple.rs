use calamp_rs::Message;

fn main() {
    let data: [u8; 117] = [
        0x83, 0x05, 0x46, 0x34, 0x66, 0x32, 0x35, 0x01, 0x01, 0x01, 0x02, 0x3a,
        0x86, 0x5f, 0xf1, 0x3a, 0x54, 0x5f, 0xf1, 0x3a, 0x57, 0xf1, 0xe2, 0x85,
        0x78, 0xe4, 0x22, 0xd6, 0x40, 0x00, 0x01, 0x36, 0xf8, 0x00, 0x00, 0x00,
        0x0b, 0x00, 0x00, 0x06, 0x20, 0x00, 0x00, 0xff, 0x8d, 0x02, 0x1e, 0x1e,
        0x00, 0x7b, 0x21, 0x10, 0x00, 0x00, 0x00, 0x31, 0xe0, 0x00, 0x00, 0x10,
        0x1a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x2a, 0x32, 0x00, 0x00, 0x03,
        0xf1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xc8, 0x2d,
        0x3f, 0x01, 0xc8, 0x2d, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let msg = Message::parse(&data);

    println!("Message Type: {}", msg.message_header.message_type);
}
