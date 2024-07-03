mod db;
mod handlers;
mod models;

use axum::extract::Extension;
use axum::{routing::get, routing::post, Router};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;


#[tokio::main] // async handling with tokio 
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_db(&database_url)
        .await
        .expect("failed to initialize database");
    let pool = Arc::new(pool);

    let app = Router::new()
        .route("/custom_details", post(handlers::create_custom_detail))
        .route("/custom_details", get(handlers::get_custom_details))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    eprintln!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

/*// testing db connection 
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

}*/




}
