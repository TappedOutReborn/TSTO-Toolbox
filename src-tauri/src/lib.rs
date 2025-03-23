// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::io;

use io::Read;
use io::Seek;
use io::Write;

// Titlebar

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

// CRC32

const POLYNOMIAL: u32 = 0xEDB88320; // CRC-32 polynomial

#[tauri::command]
fn calculate_crc32(filedata: Vec<u8>) -> u32 {
    let mut crc: u32 = !0;

    for &byte in &filedata {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if (crc & 1) != 0 {
                (crc >> 1) ^ POLYNOMIAL
            } else {
                crc >> 1
            };
        }
    }

    !crc
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![window_close, window_minimize, window_drag, calculate_crc32])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
