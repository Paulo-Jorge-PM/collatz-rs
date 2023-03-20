#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod utils;
use utils::collatz::calculate_collatz;
use utils::collatz::Number;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!(">>> {} test", 32);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_collatz(start_number: usize) -> Vec<Number> {
    return calculate_collatz(start_number);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, generate_collatz])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
