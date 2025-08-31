use crate::{HttpRequest,HttpResponse};
use serde::{Deserialize,Serialize};
use serde_json;

#[derive(Debug,Deserialize)]
struct RegisterRequest{
     username: String,
     email: String,
     password: String,    
}

#[derive(Debug,Serialize)]
struct RegisterResponse{
     status: String,
     message: String,
}

pub fn handle_register(request: &HttpRequest) -> HttpResponse{
     let register_data: Result<RegisterRequest,_> = serde_json::from_slice(&request.body);

     match register_data{
          Ok(_) => {
               let response_data = RegisterResponse{
                    status: "success".to_string(),
                    message: "Registeration Successful!".to_string(),
               };
               let response_body = serde_json::to_vec(&response_data).unwrap();

               let mut response = HttpResponse::new(200,"OK");
               response.headers.insert("Content-Type".to_string(),"application/json".to_string());
               response.body = response_body;
               response
          }
          Err(_) => {
               let response_data = RegisterResponse{
                    status: "error".to_string(),
                    message: "Invalid Registeration Request".to_string(),
               };
               let response_body = serde_json::to_vec(&response_data).unwrap();

               let mut response = HttpResponse::new(400,"Bad Request");
               response.headers.insert("Content-Type".to_string(),"application/json".to_string());
               response.body = response_body;
               response
          }
     }
}