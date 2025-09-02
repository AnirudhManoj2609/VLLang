use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::{thread};
use std::collections::HashMap;
use std::sync::Arc;
use std::fs::File;
use std::path::Path;

#[macro_use]
extern crate lazy_static;

mod database;
mod controller;//tells rustc to look for controller.rs or mod.rs inside controller directory    

struct HttpRequest{
    method: String,
    path: String,
    version: String,
    headers: HashMap<String,String>,
    body: Vec<u8>,
}

struct HttpResponse{
    status_code: u16,
    status_text: String,
    headers: HashMap<String,String>,
    body: Vec<u8>,
}
//Allows to create characteristics for the struct
impl HttpResponse{
    fn new(status_code: u16,status_text: &str) -> Self{
        HttpResponse { 
            status_code, 
            status_text: status_text.to_string(), 
            headers: HashMap::new(), 
            body: Vec::new(),
        }
    }

    fn to_bytes(&self) -> Vec<u8>{
        let mut response_string = format!("HTTP/1.1 {} {}\r\n",self.status_code,self.status_text);
        for (key,value) in &self.headers{
            response_string.push_str(&format!("{}: {}\r\n",key,value));
        }
        response_string.push_str("\r\n");

        let mut response_bytes = response_string.into_bytes();
        response_bytes.extend_from_slice(&self.body);
        response_bytes
    }
}

type Handler = fn(&HttpRequest) -> HttpResponse;

lazy_static! {
    static ref ROUTES: Arc<HashMap<&'static str,HashMap<&'static str,Handler>>> = {
        let mut routes = HashMap::new();

        /*         
        let mut login_handlers = HashMap::new();
        login_handlers.insert("POST",controller::login::handle_login as Handler);
        routes.insert("/login",login_handlers);
        */
        let mut register_handlers = HashMap::new();
        register_handlers.insert("POST",controller::register::handle_register as Handler);
        routes.insert("/register",register_handlers);

        Arc::new(routes)//adding a semicolon makes the block return nothing
    };

}

fn parse_http_request(request_str: &str) -> Option<HttpRequest> {
    let mut lines = request_str.lines();
    let first_line = lines.next()?;
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 3 {
        return None;
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();
    let version = parts[2].to_string();

    let mut headers = HashMap::new();
    let mut body_start = None;

    for (i, line) in lines.enumerate() {
        if line.is_empty() {
            body_start = Some(i + 2); // +2 for the first line and the empty line itself
            break;
        }
        let header_parts: Vec<&str> = line.splitn(2, ':').collect();
        if header_parts.len() == 2 {
            headers.insert(
                header_parts[0].trim().to_string(),
                header_parts[1].trim().to_string(),
            );
        }
    }

    let body_str = if let Some(start) = body_start {
        request_str.lines().skip(start).collect::<Vec<&str>>().join("\n")
    } else {
        String::new()
    };
    
    Some(HttpRequest {
        method,
        path,
        version,
        headers,
        body: body_str.into_bytes(),
    })
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];

    if let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            println!("Client disconnected");
            return;
        }

        let request_str = String::from_utf8_lossy(&buffer[..size]);
        println!("Raw Request: {}", request_str);

        if let Some(request) = parse_http_request(&request_str) {
            let response = find_and_run_handler(&request);
            if let Err(e) = stream.write_all(&response.to_bytes()) {
                eprintln!("Failed to write to client: {}", e);
            }
        } else {
            let response = HttpResponse::new(400, "Bad Request");
            stream.write_all(&response.to_bytes()).unwrap();
        }
    } else {
        eprintln!("Failed to read from client");
    }
}
fn find_and_run_handler(request: &HttpRequest) -> HttpResponse {
    if let Some(method_map) = ROUTES.get(request.path.as_str()) {
        if let Some(handler) = method_map.get(request.method.as_str()) {
            return handler(request);
        }
    }
    return handle_static_file(request);
}
fn guess_mime_type(path: &str) -> &str{
    if path.ends_with(".html"){
        "text/html"
    }
    else if path.ends_with(".css"){
        "text/css"
    }
    else if path.ends_with(".js"){
        "application/javascript"
    }
    else if path.ends_with("json"){
        "application/json"
    }
    else{
        "text/plain"
    }
}
fn handle_static_file(request: &HttpRequest) -> HttpResponse{
    let mut file_path = request.path.clone();

    if file_path == "/"{
        file_path = "/index.html".to_string();
    }
    let full_path = format!("./static{}",file_path);
    let path = Path::new(&full_path);

    if let Ok(mut file) = File::open(&path){
        let mut file_contents = Vec::new();
        match file.read_to_end(&mut file_contents){
            Ok(_) => {
                let mut response = HttpResponse::new(200,"OK");
                response.headers.insert("Content-Type".to_string(),guess_mime_type(&full_path).to_string());
                response.body = file_contents;
                response
            }
            Err(_) => {
                let mut response = HttpResponse::new(500,"Internal Server Error");
                response.body = b"<h1>500 Internal Server Error!We Sincerely apologies</h1>".to_vec();
                response
            }
        }
    }
    else{
        let mut response = HttpResponse::new(404,"Not Found");
        response.body = b"<h1>404 Not Found</h1>".to_vec();
        response
    }
}

fn main() -> std::io::Result<()>{

    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(database::connection::create_db_pool()){
        Ok(pool) => {
            println!("Database connection successful!");
            database::connection::set_global_pool(pool);
        }
        Err(e) => {
            println!("Database connection failed: {}",e);
        }
    }

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Connection successful!");

    for stream in listener.incoming(){
        let stream = stream?;
        //Generates a thread to deal with the particular new incoming request
        thread::spawn(||handle_client(stream));
    }

    Ok(())
}