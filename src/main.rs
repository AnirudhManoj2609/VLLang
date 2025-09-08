use std::collections::HashMap;

use crate::enums::{HttpMethod, HttpVersion};

mod database;
mod enums;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl HttpResponse {
    fn new(status_code: u16, status_text: &str) -> Self {
        HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            headers: HashMap::new(),
            body: Some(Vec::new()),
        }
    }

    // fn to_bytes(&self) -> Vec<u8> {
    //     let mut response_string = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
    //     for (key, value) in &self.headers {
    //         response_string.push_str(&format!("{}: {}\r\n", key, value));
    //     }
    //     response_string.push_str("\r\n");

    //     let mut response_bytes = response_string.into_bytes();

    //     if let Some(body) = &self.body {
    //         response_bytes.extend_from_slice(body);
    //     }

    //     response_bytes
    // }
}

fn main() -> std::io::Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    // unwrap() crashes when tokio runtime is unable to start

    match rt.block_on(database::connection::create_db_pool()) {
        Ok(pool) => {
            println!("Database connection successful!");
            database::connection::set_global_pool(pool);
        }
        Err(e) => {
            println!("Database connection failed: {}", e);
        }
    }

    Ok(())
}
