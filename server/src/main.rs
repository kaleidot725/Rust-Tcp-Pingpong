use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("START");
    let mut receive_buffer = [0; 128];
    loop {
        match stream.read(&mut receive_buffer) {
            Ok(_) => {
                println!("Result: {}", receive_buffer.iter().map(|x| format!("{:02X}", x)).collect::<String>());
                continue
            }
            Err(_) => {
                println!("ERROR");
                break
            }
        }
    }
    println!("END");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:30000")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}