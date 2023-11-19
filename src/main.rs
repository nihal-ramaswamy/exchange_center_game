use axum::{
    routing::get,
    Router,
};

pub mod dto;
pub mod utils;
pub mod unittests;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/healthcheck", 
                                  get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
