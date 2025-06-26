pub mod users;

use std::error::Error;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[derive(Debug, Clone)]
pub struct DBClient {
    pub pool: Pool<Postgres>,
}

impl DBClient {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        println!("Connecting to database...");
        let pool = PgPoolOptions::new()
            .min_connections(10)
            .connect(database_url)
            .await
            .map_err(|e| {
                eprintln!("Failed to connect to the database: {e}",);
                e
            })?;
        println!("Database connection established!");

        Ok(Self { pool })
    }
}
