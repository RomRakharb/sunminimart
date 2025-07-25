use axum::{Router, routing::get};
use server::{get_items, sync_database};

#[tokio::main]
async fn main() {
    sync_database().await;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/items", get(get_items));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
