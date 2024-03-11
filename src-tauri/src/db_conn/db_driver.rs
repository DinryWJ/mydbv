use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use super::dynamic_row::DynamicRow;

pub trait DbDriver {
    fn refresh_database(&mut self) -> Option<Vec<String>>;
    fn refresh_tables(&self) -> Option<Vec<String>>;
    fn ddl(&self, table_name: &str) -> String;
    fn exec_result<'a>(&self, db: &'a str, query: &'a str) -> Vec<DbResult>;
}

#[derive(Debug, Serialize)]
pub struct DbResult {
    pub start_time: u128,
    pub end_time: u128,
    pub query: String,
    pub rows: Option<Vec<DynamicRow>>,
    pub error: Option<String>,
}

impl DbResult {
    pub fn new(query: String) -> DbResult {
        Self {
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            end_time: 0,
            query: query,
            rows: None,
            error: None,
        }
    }
}
