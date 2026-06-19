use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use std::env;

#[derive(Deserialize)]
struct UserPayload {
    name: String,
    email: String,
}

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
