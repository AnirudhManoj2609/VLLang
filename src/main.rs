use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::net::SocketAddr;

// AppState holds the database connection pool.
// It's a shared state accessible by all handlers.
#[derive(Clone)]
struct AppState {
    db_pool: Pool<Postgres>,
}

// Struct for the user data we get from the database.
#[derive(sqlx::FromRow, Serialize)]
struct User {
    id: i32,
    username: String,
    #[serde(skip_serializing)] // Don't send the hash to the client
    password: String,
}

// Struct for the signup request payload.
#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

// Struct for the login request payload.
#[derive(Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool.");

    // Create the shared state
    let app_state = AppState { db_pool };

    // Define the application routes (endpoints)
    let app = Router::new()
        .route("/signup", post(signup_handler))
        .route("/login", post(login_handler))
        .with_state(app_state); // Provide the state to the handlers

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn signup_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>, // axum deserializes the request body into CreateUser
) -> impl IntoResponse {
    // Hash the password
    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password").into_response();
        }
    };

    // Insert the new user into the database
    let result = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
        payload.username,
        password_hash
    )
    .fetch_one(&state.db_pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "User created successfully").into_response(),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            (StatusCode::CONFLICT, "Username already exists").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response(),
    }
}

/// ## Login Handler
async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> impl IntoResponse {
    // Find the user by username
    let user = match sqlx::query_as!(
        User,
        "SELECT id, username, password FROM users WHERE username = $1",
        payload.username
    )
    .fetch_optional(&state.db_pool)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response();
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    // Verify the password
    let valid_password = match bcrypt::verify(&payload.password, &user.password) {
        Ok(valid) => valid,
        Err(_) => false,
    };

    if valid_password {
        (StatusCode::OK, "Login successful").into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response()
    }
}
