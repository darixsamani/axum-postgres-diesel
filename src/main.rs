use axum::{Json, Router, routing::get};
use crate::models::product::Product;

pub mod models;


async fn get_product() -> Json<Product>{
    Json(Product{
        id: 1,
        name: "Rust Book".to_string()
    })
} 

async fn create_product(Json(payload): Json<Product>) -> Json<Product>{
    Json(payload)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new()
    .route("/product", get(get_product).post(create_product));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
