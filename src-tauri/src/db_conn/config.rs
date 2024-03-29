use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    io::Read, path::PathBuf,
};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbConfig {
    pub id: u32,
    pub nickname: String,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub sql_type: String,

    pub databases: Option<Vec<String>>,
}

impl DbConfig {
    pub fn new(
        id: u32,
        nickname: String,
        host: Option<String>,
        port: Option<u16>,
        user: Option<String>,
        password: Option<String>,
        database: Option<String>,
        sql_type: String,
    ) -> Self {
        Self {
            id,
            nickname,
            host,
            port,
            user,
            password,
            database,
            databases: None,
            sql_type,
        }
    }

    pub fn load() -> Result<Vec<DbConfig>, Box<dyn Error>> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let mut file_path = PathBuf::from(manifest_dir);
        file_path.push("config");
        file_path.push("db.json");
    
        println!("load db config from: {:?}", file_path);
        
        let mut file = File::open(&file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let configs: Vec<DbConfig> = serde_json::from_str(&contents)?;

        Ok(configs)
    }
}
