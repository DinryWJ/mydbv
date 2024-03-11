// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{sync::Arc};

use crossbeam::channel::unbounded;
use dashmap::DashMap;
use db_conn::{config::DbConfig, db_driver::DbDriver, mysql_driver::MysqlDriver};
use mydbv::{Message, ThreadKeeper};

pub mod db_conn;

pub struct DbState {
    pub rw: Arc<DashMap<String, ThreadKeeper>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_config() -> Result<String, String> {
    let dbs = DbConfig::load();
    match dbs {
        Ok(dbs) => Ok(serde_json::to_string(&dbs).unwrap()),
        Err(e) => {
            println!("err {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn new_conn(
    uid: String,
    id: u32,
    nickname: String,
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    sql_type: String,
    state: tauri::State<DbState>,
) -> Result<String, String> {
    let w = state.rw.get(&uid);
    if w.is_some() {
        state.rw.remove(&uid);
    }

    let config = DbConfig::new(id, nickname, host, port, user, password, database, sql_type);

    let (s, r) = unbounded();
    // let id = id;
    let s2 = s.clone();
    let r2 = r.clone();
    let keeper = ThreadKeeper::new(id, s, r, move || {
        let driver = MysqlDriver::new(&config.clone());
        match driver {
            Ok(d) => loop {
                let message = r2.recv().unwrap();
                println!("receive message: {}", message);
                let message: Message =
                    serde_json::from_str(&message).expect("Failed to parse Message");
                let res = d.exec_result(&message.db, &message.sql);
                let res = serde_json::to_string(&res).unwrap();
                s2.send(res).unwrap();
            },
            Err(e) => {
                let message = format!("connect failed: {}", e);
                s2.send(message).unwrap()
            }
        }
    });

    state.rw.insert(uid, keeper);
    Ok("ok".to_string())
}

#[tauri::command]
fn query(uid: String, db: String, sql: String, state: tauri::State<DbState>) -> Result<String, String> {
    let w = state.rw.get(&uid).unwrap();

    match w.send_message(Message {
        db,
        sql,
    }) {
        Ok(message) => Ok(message),
        Err(_) => Err("error".to_string()),
    }
}


#[tauri::command]
fn show_databases(uid: String, state: tauri::State<DbState>) -> Result<String, String> {
    let w = state.rw.get(&uid).unwrap();

    match w.send_message(Message {
        db: "mysql".to_string(),
        sql: "show databases".to_string(),
    }) {
        Ok(message) => Ok(message),
        Err(_) => Err("error".to_string()),
    }
}

fn main() {
    let map = DashMap::new();
    let state = DbState { rw: Arc::new(map) };
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            new_conn,
            query
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
