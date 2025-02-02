// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

#[tauri::command]
fn window_close(window: tauri::Window) {
    window.close().unwrap();
}

#[tauri::command]
fn window_minimize(window: tauri::Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn window_drag(window: tauri::Window) {
    window.start_dragging().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, window_close, window_minimize, window_drag])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
