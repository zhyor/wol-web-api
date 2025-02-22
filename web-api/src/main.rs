use axum::{extract::Query, routing::get, Router};
use std::collections::HashMap;
use wol::wol;

async fn wake_on_lan(Query(params): Query<HashMap<String, String>>) -> String {
    if let Some(mac) = params.get("mac") {
        match wol::wake_on_lan(&mac) {
            Some(()) => return format!("Success: {}", mac),
            None => return format!("Mac Format Error: {}", mac),
        }
    } else {
        "No MAC Provided".to_string()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/wol", get(wake_on_lan))
        .route("/", get("Working"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:56767").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
