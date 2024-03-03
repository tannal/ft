# File Transfer

A simple file transfer utility written in Rust that can operate in either server or client mode to send and receive files over TCP.

## Features

- **Server Mode**: Listens for incoming TCP connections and saves received files to a specified path.
- **Client Mode**: Connects to a server and sends a specified file.

## Prerequisites

- Rust Programming Language: Ensure you have Rust installed on your system. Visit [https://rustup.rs/](https://rustup.rs/) for installation instructions.

## Building the Project

To build the project, navigate to the root directory of the project and run:

```bash
cargo build
```

```bash
cargo run -- server 7878 received_file.txt
```

```bash
cargo run -- client 127.0.0.1:7878 file_to_send.txt
```