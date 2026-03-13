use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

// Struct untuk format Response JSON
#[derive(Serialize)]
struct Message {
    status: String,
    message: String,
}

#[tokio::main]
async fn main() {
    // Membangun route aplikasi
    let app = Router::new().route("/", get(hello_world));

    // Menentukan alamat server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server berjalan di http://{}", addr);

    // Menjalankan server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Perbaikan Error E0282: Axum serve memerlukan router yang jelas
    axum::serve(listener, app).await.unwrap();
}

// Handler untuk merespon dengan JSON
async fn hello_world() -> Json<Message> {
    let response = Message {
        status: "success".to_string(),
        message: "Hello World from Rust!".to_string(),
    };

    Json(response)
}
