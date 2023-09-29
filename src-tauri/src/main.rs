// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod sql_handler;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

fn main() {
  let mut _pool = sql_handler::sql_setup();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
