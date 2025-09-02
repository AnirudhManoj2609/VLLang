use crate::{HttpRequest,HttpResponse};
use crate::database::connection::get_global_pool;
use bcrypt::{hash,DEFAULT_COST};
use tokio;
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
          Ok(data) => {
               let pool = get_global_pool();

               let result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {

                         let user_exists: bool = match sqlx::query_scalar!(
                              "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 or email = $2)",
                              data.username,
                              data.email,
                         )
                         .fetch_one(pool)
                         .await
                         {
                              Ok(Some(exists)) => exists,
                              Ok(None) => false,
                              Err(e) => return Err(e),
                         };

                         if user_exists{
                              return Ok("user_exists");
                         }

                         let hashed_password = match hash(&data.password,DEFAULT_COST){
                              Ok(hashed) => hashed,
                              Err(_) => return Ok("hashing_error"),
                         };

                         match sqlx::query!(
                              "INSERT INTO users (username,email,password) VALUES ($1,$2,$3)",
                              data.username,
                              data.email,
                              hashed_password,
                         )
                         .execute(pool)
                         .await
                         {
                              Ok(_) => Ok("success"),
                              Err(e) => Err(e),
                         }
                    })
               });
               match result{
                    Ok("success") => {
                         let response_data = RegisterResponse{
                              status: "success".to_string(),
                              message: "User inserted successfully".to_string(),
                         };
                         create_json_response(200, "OK", response_data)
                    }
                    Ok("user_exists") => {
                         let response_data = RegisterResponse{
                              status: "error".to_string(),
                              message: "User already exists".to_string(),
                         };
                         create_json_response(400, "Bad Request", response_data)
                    }
                    Ok("hashing_error") => {
                         let response_data = RegisterResponse{
                              status: "error".to_string(),
                              message: "Password processing error".to_string(),
                         };
                         create_json_response(500, "Internal Server Error", response_data)
                    }
                    Ok(_) => {  // ← Add this wildcard pattern
                         let response_data = RegisterResponse{
                              status: "error".to_string(),
                              message: "Unexpected error".to_string(),
                         };
                         create_json_response(500, "Internal Server Error", response_data)
                    }
                    Err(_) => {
                         let response_data = RegisterResponse{
                              status: "error".to_string(),
                              message: "Database Error".to_string(),
                         };
                         create_json_response(500, "Internal Server Error", response_data)
                    }
               }
          }
          Err(_) => {
               let response_data = RegisterResponse{
                    status: "error".to_string(),
                    message: "Invalid Registeration Request".to_string(),
               };
               create_json_response(500,"Internal Server Error",response_data)
          }
     }
}

fn create_json_response(status_code: u16, status_text: &str, data: RegisterResponse) -> HttpResponse {
    let response_body = serde_json::to_vec(&data).unwrap();
    
    let mut response = HttpResponse::new(status_code, status_text);
    response.headers.insert("Content-Type".to_string(), "application/json".to_string());
    response.body = response_body;
    response
}