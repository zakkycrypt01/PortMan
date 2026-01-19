mod port_monitor;
mod gui;

use gui::{GuiBackend, PortInfo};
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Build router
    let app = Router::new()
        .route("/api/ports", get(get_ports))
        .route("/api/ports/:port/kill", post(kill_port))
        .route("/api/ports/:port/details", get(get_port_details))
        .fallback(get(serve_index))
        .layer(CorsLayer::permissive());

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5173")
        .await
        .expect("Failed to bind to port 5173");
    
    println!("🎨 PortMan GUI starting at http://127.0.0.1:5173");
    println!("Press Ctrl+C to stop");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn get_ports() -> Json<Result<Vec<PortInfo>, String>> {
    let backend = GuiBackend::new();
    match backend.get_all_ports() {
        Ok(ports) => Json(Ok(ports)),
        Err(e) => Json(Err(e.to_string())),
    }
}

async fn kill_port(Path(port): Path<u16>) -> (StatusCode, Json<serde_json::Value>) {
    let backend = GuiBackend::new();
    match backend.kill_process(port) {
        Ok(msg) => (StatusCode::OK, Json(json!({ "success": true, "message": msg }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e.to_string() }))),
    }
}

async fn get_port_details(Path(port): Path<u16>) -> Json<Result<Option<gui::PortDetails>, String>> {
    let backend = GuiBackend::new();
    match backend.get_port_details(port) {
        Ok(details) => Json(Ok(details)),
        Err(e) => Json(Err(e.to_string())),
    }
}

async fn serve_index() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>PortMan</title>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <link rel="stylesheet" href="styles.css">
    </head>
    <body>
        <div id="app"></div>
        <script src="app.js"></script>
    </body>
    </html>
    "#
}

