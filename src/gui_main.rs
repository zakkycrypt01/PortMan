mod port_monitor;
mod gui;

use gui::{GuiBackend, PortInfo};
use tauri::Manager;

#[tauri::command]
fn get_all_ports() -> Result<Vec<PortInfo>, String> {
    let backend = GuiBackend::new();
    backend.get_all_ports().map_err(|e| e.to_string())
}

#[tauri::command]
fn kill_port(port: u16) -> Result<String, String> {
    let backend = GuiBackend::new();
    backend.kill_process(port).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_port_details(port: u16) -> Result<Option<gui::PortDetails>, String> {
    let backend = GuiBackend::new();
    backend.get_port_details(port).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_all_ports, kill_port, get_port_details])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
