use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;

fn parse_http_request(request: &str) -> Option<(String, String, String)> {
    if let Some(first_line) = request.split("\r\n").next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 3 {
            let method = parts[0].to_string();
            let path = parts[1].to_string();
            let version = parts[2].to_string();
            return Some((method, path, version));
        }
    }
    None
}

fn handle_request(method: &str,path: &str,version: &str){
    match path{
        "/login" => controller::login::handle_login(method,path,version);//Ashwins function name inside controller folder
        "/register" => controller::register::handle_register(method,path,version);
        _ =>println!("404 Not found {}",path);
    }
}
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(0) => {
            // Client closed connection
            println!("Client disconnected");
            return;
        }
        Ok(size) => {
            let request_str = String::from_utf8_lossy(&buffer[..size]);
            println!("Raw Request: {}",request_str);
            
            if let Some((method,path,version)) = parse_http_request(&request_str){
                handle_request(&method,&path,&version);
            }
            else{
                println!("Failed to parse the request!");
            }

            if let Err(e) = stream.write_all(&buffer[..size]) {
                eprintln!("Failed to write to client: {}", e);
                break;
            }
        }
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
            return;
        }
    }
}


fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Connection successful!");

    for stream in listener.incoming(){
        let stream = stream?;
        //Generates a thread to deal with the particular new incoming request
        thread::spawn(||handle_client(stream));
    }

    Ok(())
}