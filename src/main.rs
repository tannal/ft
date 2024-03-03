use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} server <PORT> <FILENAME>", args[0]);
        eprintln!("  {} client <ADDRESS> <FILEPATH>", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "server" => {
            if args.len() != 4 {
                eprintln!("Usage: {} server <PORT> <FILENAME>", args[0]);
                return Ok(());
            }
            let port = &args[2];
            let filename = &args[3];
            run_server(port, filename)?;
        }
        "client" => {
            if args.len() != 4 {
                eprintln!("Usage: {} client <ADDRESS> <FILEPATH>", args[0]);
                return Ok(());
            }
            let address = &args[2];
            let filepath = &args[3];
            run_client(address, filepath)?;
        }
        _ => eprintln!("Invalid command: {}", args[1]),
    }

    Ok(())
}

fn run_server(port: &str, filename: &str) -> io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    println!("Server listening on port {}", port);

    match listener.accept() {
        Ok((mut stream, addr)) => {
            println!("Client {} connected", addr);
            let mut file = File::create(filename)?;
            let mut buffer = [0; 1024];
            while let Ok(size) = stream.read(&mut buffer) {
                if size == 0 {
                    break;
                }
                file.write_all(&buffer[..size])?;
            }
            println!("File received successfully!");
        }
        Err(e) => eprintln!("Connection failed: {}", e),
    }

    Ok(())
}

fn run_client(address: &str, filepath: &str) -> io::Result<()> {
    let mut file = File::open(filepath)?;
    let mut stream = TcpStream::connect(address)?;

    let mut buffer = [0; 1024];
    while let Ok(size) = file.read(&mut buffer) {
        if size == 0 {
            break;
        }
        stream.write_all(&buffer[..size])?;
    }

    println!("File sent successfully!");
    Ok(())
}
