use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client closed connection
                println!("Client disconnected");
                break;
            }
            Ok(size) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
                if let Err(e) = stream.write_all(&buffer[..size]) {
                    eprintln!("Failed to write to client: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                break;
            }
        }
    }
}


fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Connection successful!");

    for stream in listener.incoming(){
        let stream = stream?;
        thread::spawn(||handle_client(stream));
    }

    Ok(())
}