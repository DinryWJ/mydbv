// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use dashmap::DashMap;

mod command;
mod db_conn;
mod interface;

fn main() {
    let map = DashMap::new();
    let state = command::DbState { rw: Arc::new(map) };
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            interface::cmd_config::load_config,
            interface::cmd_sql::new_conn,
            interface::cmd_sql::query,
            interface::cmd_sql::specific_query,
            interface::cmd_sql::destory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
