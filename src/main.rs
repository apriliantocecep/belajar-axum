use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use serde::{Deserialize};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route(
            "/hello",
            get(hello_handler),
        )
        .route("/hello2/{name}", get(hello_handler_path));

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, routes_hello).await.unwrap()
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <b>{name}</b>"))
}

async fn hello_handler_path(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <b>{name}</b>"))
}