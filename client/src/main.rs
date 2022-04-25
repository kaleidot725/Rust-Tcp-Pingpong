use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.0.1:30000")?;
    let send_buffer = b"PING_PONG";
    let send_size = stream.write(send_buffer);
    return match send_size {
        Ok(_) => {
            let mut receive_buffer = [0; 128];
            let receive_size = stream.read(&mut receive_buffer);
            match receive_size {
                Ok(_) => {
                    Ok(())
                }
                Err(error) => { Err(error) }
            }
        }
        Err(error) => { Err(error) }
    }
}
