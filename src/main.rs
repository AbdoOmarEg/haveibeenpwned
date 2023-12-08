use axum::routing::get;
use axum::Router;

mod migrate;
mod web;

use migrate::initialize_database;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use web::{accept_form, handler_404, show_form};

// #[derive(Clone)]
// struct AppState {
//     pool: SqlitePool,
// }

#[tokio::main]
async fn main() {
    // Initialize the database and run migrations
    let pool = initialize_database().await;

    // Set up routes and run the application
    let app = Router::new()
        .route("/", get(show_form).post(accept_form))
        .fallback(handler_404)
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
