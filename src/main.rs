use std::net::SocketAddr;
use axum::{extract::{Path, Query}, response::Html, routing::get, Router};
use serde::Deserialize;

pub use self::error::{Error, Result};
pub use web::routes_login;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

mod error;
mod web;

#[tokio::main]
async fn main() {
    let login_routes = web::routes_login::routes_login();
    
    let hello_routes = Router::new()
        .route("/hello", get(hello))
        .route("/hello/{name}", get(hello_with_name));
    
    let routes_all = Router::new()
        .merge(login_routes) 
        .merge(hello_routes); 

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, routes_all).await.unwrap();
}

async fn hello(Query(params): Query<HelloParams>) -> impl axum::response::IntoResponse {
    println!("{:<12} - Hello World!", "GET /hello");
    
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello, {name}</h1>"))
}

async fn hello_with_name(Path(name): Path<String>) -> impl axum::response::IntoResponse {
    println!("{:<12} - Hello {name}!", "GET /hello/{name}");
    Html(format!("<h1>Hello, {name}</h1>"))
}