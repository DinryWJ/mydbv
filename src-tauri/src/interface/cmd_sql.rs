use crate::{
    command::DbState,
    db_conn::{config::DbConfig, message::Message, thread_keeper::ThreadKeeper},
};
use crossbeam::channel::unbounded;


#[tauri::command]
pub fn new_conn(
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

    let (tx_to_subthread, rx_from_main) = unbounded();
    let (tx_to_main, rx_from_subthread) = unbounded();

    let keeper = ThreadKeeper::new(
        id,
        tx_to_subthread,
        rx_from_main,
        tx_to_main,
        rx_from_subthread,
        config,
    );

    state.rw.insert(uid, keeper);
    Ok("ok".to_string())
}

#[tauri::command]
pub fn query(
    uid: String,
    db: String,
    sql: String,
    state: tauri::State<DbState>,
) -> Result<String, String> {
    let w = state.rw.get(&uid).unwrap();

    match w.send_message(Message { db, sql }) {
        Ok(message) => Ok(message),
        Err(_) => Err("error".to_string()),
    }
}

#[tauri::command]
pub fn specific_query(
    uid: String,
    db: Option<String>,
    q_type: u8,
    state: tauri::State<DbState>,
) -> Result<String, String> {
    let w = state.rw.get(&uid).unwrap();

    let mut message: Option<Message> = None;
    if q_type == 1 {
        message = Some(Message {
            db: "mysql".to_string(),
            sql: "show databases".to_string(),
        });
    } else if q_type == 2 {
        message = Some(Message {
            db: db.unwrap(),
            sql: "show tables".to_string(),
        });
    }

    if message.is_none() {
        return Err("error".to_string());
    }

    match w.send_message(message.unwrap()) {
        Ok(message) => Ok(message),
        Err(_) => Err("error".to_string()),
    }
}

#[tauri::command]
pub fn destory(uid: String, state: tauri::State<DbState>) -> Result<String, String> {
    let w = state.rw.get(&uid).unwrap();
    state.rw.remove(&uid);

    match w.destory() {
        Ok(message) => Ok(message),
        Err(_) => Err("error".to_string()),
    }
}
