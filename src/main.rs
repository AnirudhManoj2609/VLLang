use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load environment variables from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Sample data
    let fullname = "John Doe";
    let username = "johnd";
    let email = "johnd@example.com";
    let phone_number = Some("+911234567890"); // Option<String> so it can be NULL

    // Insert query
    sqlx::query!(
        r#"
        INSERT INTO USERS (fullname, username, email, phone_number)
        VALUES ($1, $2, $3, $4)
        "#,
        fullname,
        username,
        email,
        phone_number
    )
    .execute(&pool)
    .await?;

    println!("User inserted successfully!");
    Ok(())
}
