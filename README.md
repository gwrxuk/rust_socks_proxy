
# Rust SOCKS5 Proxy Server

This project is a simple implementation of a SOCKS5 proxy server built in Rust. It uses `tokio` for asynchronous IO operations, making it capable of handling multiple client connections efficiently. The current implementation supports the CONNECT command with no authentication, making it suitable for basic forwarding purposes.

## Features

- SOCKS5 CONNECT command support
- Handles IPv4 and domain name addresses
- Asynchronous processing of client requests

## Requirements

- Rust 1.58 or newer

## Installation

To get started with the Rust SOCKS5 Proxy Server, you will need to have Rust and Cargo installed on your machine. If you don't have them installed, please follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

1. Clone the repository:

```sh
git clone https://github.com/gwrxuk/rust_socks_proxy.git
cd rust_socks_proxy
```

2. Build the project:

```sh
cargo build --release
```

3. Run the proxy server:

```sh
cargo run --release
```

The server will start listening on `127.0.0.1:1080` by default.

## Usage

Configure your client to use the SOCKS5 proxy by setting the proxy server address to `127.0.0.1` and the port to `1080`. No authentication is required.

## Contributing

Contributions to the Rust SOCKS5 Proxy Server are welcome! Feel free to open issues or pull requests to improve the project.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

