# Calamp-rs

[![Banner](https://m0x.ru/pics/calamp_rs_github.png)](https://github.com/murilobsd/calamp-rs)

[![License: MIT](https://img.shields.io/badge/LICENSE-MIT-blue?style=for-the-badge)](./LICENSE) 
[![Crates.io Version](https://img.shields.io/crates/v/calamp-rs.svg?style=for-the-badge)](https://crates.io/crates/calamp-rs) 
[![Doc](https://img.shields.io/badge/CALAMPRS-DOC-blue?style=for-the-badge)](https://docs.rs/calamp-rs)

## Features

- __Fast__: use [nom](https://crates.io/crates/nom) parser combinator
- __Security__: memory safe
- __Messages__: support EventReport

## Quickstart

To start with calamp-rs, add the following to your `Cargo.toml` file:

```toml
[dependencies]
calamp-rs= "0.1.4"
```

Next, parse message:

```rust
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
```

Finally, run this benchmark with `cargo run`. You should see output similar to
the following:

```
Message Type: MessageType::EventReport
```

## Examples

To see some examples just run them via `cargo run --examples`.

```
cargo run --example srv_udp
```

Another terminal:

```
cargo run --example cli_udp
```

## Contributing

First, thank you for contributing.

One great way to contribute to calamp-rs is to use it for your own
benchmarking needs and report your experiences, file and comment on issues,
etc.

Code or documentation improvements in the form of pull requests are also
welcome. If you're not sure what to work on, try checking the Beginner label.

If your issues or pull requests have no response after a few days, feel free to
ping me (@muriloijanc).

For more details, see the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## License

Calamp-rs is distributed under the terms of the BSD2 license.

See [LICENSE](LICENSE)
