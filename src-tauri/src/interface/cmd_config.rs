use crate::db_conn::config::DbConfig;

#[tauri::command]
pub fn load_config() -> Result<String, String> {
    let dbs = DbConfig::load();
    match dbs {
        Ok(dbs) => Ok(serde_json::to_string(&dbs).unwrap()),
        Err(e) => {
            println!("err {}", e);
            Err(e.to_string())
        }
    }
}
