mod db;

use axum::extract::Extension;
use axum::{routing::get, routing::post, Router};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_postgres::Client;

#[tokio::main] // async handling with tokio 
async fn main() {
   
dotenv().ok();

let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
let pool = db::init_db(&database_url)
.await.expect("failed to initialize database");

let pool = Arc::new(pool);

// testing db connection 
test_db_connection(&pool).await;

async fn test_db_connection(pool: &Arc<Client>) {
    let stmt = "SELECT 1";
    match pool.query_one(stmt, &[]).await {
        Ok(row) => {
            let result: i32 = row.get(0);
            if result == 1 {
                eprintln!("Database connection successful");
            }
        }
    
    Err(e) => {
        eprintln!("database connection failed {}", e);
    }

}

}


}
