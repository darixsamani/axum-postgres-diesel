use axum::{Json, Router, routing::get};
use crate::schemas::product::Product;

use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod schemas;


async fn get_product() -> Json<Product>{
    Json(Product{
        id: 1,
        name: "Rust Book".to_string()
    })
} 

async fn create_product(Json(payload): Json<Product>) -> Json<Product>{
    Json(payload)
}

#[tokio::main]
async fn main() {

    // 1. Initialize the tracing subscriber to output logs to stdout
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("axum_app=debug,tower_http=debug".into()), 
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // 2. Build your router and attach the standard TraceLayer

    let app = Router::new()
    .route("/product", get(get_product).post(create_product))
    .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );
    
    // 3. Run the application
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
