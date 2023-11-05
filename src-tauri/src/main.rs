// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate mysql;
extern crate log;
extern crate env_logger;
use log::{debug, error, info, warn};
use mysql::*;
pub mod sql_handler;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![sql_handler::register_user,
                                            sql_handler::products,
                                            sql_handler::login,
                                            sql_handler::search_product])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
