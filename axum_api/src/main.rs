use axum::{body::Body, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post, delete}, Router, extract::{Path, Query, Json }};
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};


#[derive(Serialize)]
struct User{
    id: u64,
    name: String,
    email: String
}

#[derive(Deserialize)]
struct Page{
    number: u32,
}

#[derive(Deserialize)]
struct Item{
    title: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/create-user", post(create_user))
    .route("/users", get(list_users))
    .route("/item/{id}", get(show_item))
    .route("/add-item", post(add_item))
    .route("/delete-user/{user_id}", delete(delete_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    "Hello, Axum! 🦀"
}

async fn create_user() -> impl IntoResponse {
    println!("User created");
    Response::builder().status(StatusCode::CREATED).body(Body::from("User created")).unwrap()
}

async fn list_users() -> impl IntoResponse {
    let users = vec![
        User {
            id: 1,
            name: "John Doe".to_string(),
            email: "John@example.com".to_string()
        },
        User {
            id: 2,
            name: "Jane Doe".to_string(),
            email: "Jane@example.com".to_string()
        }
    ];
    Json(users)
}

async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}

async fn add_item(Json(item): Json<Item>) -> String {
    format!("{} added", item.title)
}

async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User { id: user_id, name: String::new(), email: String::new() })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        ))
    }
}

async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    if user_id % 2 == 0 {
        Ok(())
    } else {
        Err("User not found".to_string())
    }
}