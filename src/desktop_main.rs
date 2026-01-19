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
use std::process::Command;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let port = 5174;
    let addr = format!("127.0.0.1:{}", port);
    
    println!("🎨 PortMan Desktop App Starting...");
    println!("📍 Server: http://{}", addr);
    
    // Build router
    let app = Router::new()
        .route("/api/ports", get(get_ports))
        .route("/api/ports/:port/kill", post(kill_port_handler))
        .route("/api/ports/:port/details", get(get_port_details_handler))
        .fallback(get(serve_index))
        .layer(tower_http::cors::CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");
    
    println!("✅ Server started successfully!");
    
    // Try to open browser after a short delay
    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        open_browser(&format!("http://{}", addr));
    });

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

fn open_browser(url: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(url).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("start").arg(url).spawn();
    }
}

async fn get_ports() -> Json<Vec<PortInfo>> {
    let backend = GuiBackend::new();
    match backend.get_all_ports() {
        Ok(ports) => Json(ports),
        Err(_) => Json(Vec::new()),
    }
}

async fn kill_port_handler(Path(port): Path<u16>) -> (StatusCode, Json<serde_json::Value>) {
    let backend = GuiBackend::new();
    match backend.kill_process(port) {
        Ok(msg) => (StatusCode::OK, Json(json!({ "success": true, "message": msg }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e.to_string() }))),
    }
}

async fn get_port_details_handler(Path(port): Path<u16>) -> Json<Option<gui::PortDetails>> {
    let backend = GuiBackend::new();
    match backend.get_port_details(port) {
        Ok(details) => Json(details),
        Err(_) => Json(None),
    }
}

async fn serve_index() -> &'static str {
    include_str!("../ui/index.html")
}
