use std::net::SocketAddr;
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(|| async { Html("Hello, <strong>World!</strong>") }));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_hello).await.unwrap();
}