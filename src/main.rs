use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;

mod controller;//tells rustc to look for controller.rs or mod.rs inside controller directory    

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

fn handle_request(stream: &mut TcpStream,method: &str,path: &str,version: &str,body: &[u8]) -> std::io::Result<()>{
    match path{
        "/login" => controller::login::handle_login(stream,&method,&path,&version,body),//Ashwins function name inside controller folder
        "/register" => controller::register::handle_register(stream,&method,&path,&version,body),
        _ => {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found";
            stream.write_all(response.as_bytes())?;
            Ok(())
        }
    }
}

fn find_headers_end(buffer: &[u8]) -> Option<usize>{
    let double_newline = b"\r\n\r\n";
    buffer.windows(double_newline.len()).position(|window| window == double_newline)
        .map(|pos| pos + double_newline.len())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    let mut headers_end = 0;
    
    match stream.read(&mut buffer) {
        Ok(0) => {
            // Client closed connection
            println!("Client disconnected");
            return;
        }
        Ok(size) => {
            if let Some(pos) = find_headers_end(&buffer[..size]){
                headers_end = pos;
            }
            else{
                println!("Failed to find the end of the headers!");
                let response = "HTTP/1.1 400 Bad Request\r\n\r\nBad Request";
                stream.write_all(response.as_bytes()).unwrap();
                return;
            }
            let request_str = String::from_utf8_lossy(&buffer[..headers_end]);
            let body = &buffer[headers_end..size];
            println!("Raw Request: {}",request_str);
            
            if let Some((method,path,version)) = parse_http_request(&request_str){
                handle_request(&mut stream,&method,&path,&version,body);
            }
            else{
                println!("Failed to parse the request!");
            }

            if let Err(e) = stream.write_all(&buffer[..size]) {
                eprintln!("Failed to write to client: {}", e);
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