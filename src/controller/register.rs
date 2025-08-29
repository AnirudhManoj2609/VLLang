use std::io::{Read,Write};
use std::net::TcpStream;

pub fn handle_register(stream: &mut TcpStream,method: &str,path: &str,version: &str,body: &[u8]) -> std::io::Result<()>{
     println!("Handling register request!");

     let body_str = String::from_utf8_lossy(body);

     if body_str.contains("google_signup=true"){
          println!("Signup from google!");
          let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 32\r\n\r\nSigned up with Google Successfully.";
          stream.write_all(response.as_bytes());
     }
     else{
          let mut full_name = "";
          let mut email = "";
          let mut phone_number = "";
          let mut password = "";

          for line in body_str.split('&'){
               let parts: Vec<&str> = line.split('=').collect();
               if parts.len() == 2{
                    match parts[0]{
                         "full_name" => full_name = parts[1],
                         "email" => email = parts[1],
                         "phone_number" => phone_number = parts[1],
                         "password" => password = parts[1],
                         _ => {}
                    }
               }
          }
          let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 22\r\n\r\nRegisteration successful.";
          stream.write_all(response.as_bytes());
     }
     Ok(())
}