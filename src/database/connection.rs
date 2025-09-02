use sqlx::postgres::{PgPool,PgPoolOptions};
use std::env;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub fn set_global_pool(pool: PgPool){
     DB_POOL.set(pool).expect("Could not setup the pool!");
}

pub fn get_global_pool() -> &'static PgPool{
     DB_POOL.get().expect("Pool is not set yet!")
}

pub async fn create_db_pool() -> Result<PgPool,sqlx::Error>{
     
     dotenv().ok();

     let database_url = env::var("DATABASE_URL")
                         .expect("DATABASE_URL is not in the .env file");
     PgPoolOptions::new()
          .max_connections(5)
          .connect(&database_url)
          .await
}