use mysql::{from_row, prelude::Queryable, Conn, OptsBuilder, Result};
use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use super::{
    config::DbConfig,
    db_driver::{DbDriver, DbResult},
    dynamic_row::DynamicRow,
};

pub struct MysqlDriver {
    pub config: DbConfig,
    pub con: Mutex<Conn>,
    pub chooesed_db: String,
}

pub fn get_new_conn(config: &DbConfig) -> Result<Conn> {
    let opts = OptsBuilder::new()
        .user(config.user.as_ref())
        .pass(config.password.as_ref())
        .ip_or_hostname(config.host.as_ref())
        .tcp_port(config.port.unwrap_or_else(|| 3306))
        .db_name(config.database.as_ref());

    Conn::new(opts)
}

impl MysqlDriver {
    pub fn new(config: &DbConfig) -> Result<Self> {
        let con = get_new_conn(config)?;
        Ok(Self {
            config: config.clone(),
            con: Mutex::new(con),
            chooesed_db: "".to_string(),
        })
    }
}

impl DbDriver for MysqlDriver {
    fn refresh_database(&mut self) -> Option<Vec<String>> {
        let mut con = self.con.lock().unwrap();
        let query_dbs = "show databases";
        let query_dbs_stmt = con.prep(query_dbs).unwrap();
        let mut result = con.exec_iter(query_dbs_stmt, ()).unwrap();

        let mut dbs: Vec<String> = Vec::new();
        while let Some(result_set) = result.iter() {
            for row in result_set {
                let db: String = from_row(row.unwrap());
                dbs.push(db);
            }
        }
        if dbs.len() > 0 {
            Some(dbs)
        } else {
            None
        }
    }

    fn refresh_tables(&self) -> Option<Vec<String>> {
        let mut con = self.con.lock().unwrap();

        let query_dbs = "show tables";
        con.select_db(&self.chooesed_db);
        let query_dbs_stmt = con.prep(query_dbs).unwrap();
        let mut result = con.exec_iter(query_dbs_stmt, ()).unwrap();
        let mut res: Vec<String> = Vec::new();

        while let Some(result_set) = result.iter() {
            for row in result_set {
                let name: String = from_row(row.unwrap());
                res.push(name);
            }
        }

        if res.len() > 0 {
            Some(res)
        } else {
            None
        }
    }

    fn ddl(&self, table_name: &str) -> String {
        let mut con = self.con.lock().unwrap();

        let query_dbs = format!("SHOW CREATE TABLE {}", table_name);
        con.select_db(&self.chooesed_db);
        let query_dbs_stmt = con.prep(query_dbs).unwrap();
        let mut query_result = con.exec_iter(query_dbs_stmt, ()).unwrap();

        let mut second_result_set = query_result.iter().unwrap();
        let first_row = second_result_set.next().unwrap().unwrap();
        let (_, ddl): (String, String) = from_row(first_row);
        ddl
    }

    fn exec_result<'a>(&self, db: &'a str, query: &'a str) -> Vec<DbResult> {
        let mut c = get_new_conn(&self.config).unwrap();
        c.select_db(db);

        query
            .split(";")
            .filter(|s| !s.is_empty())
            .map(|query| {
                let mut result = DbResult::new(query.to_string());

                let query_result = c.query_iter(query);

                let end_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                result.end_time = end_time;

                match query_result {
                    Ok(query_result) => {
                        let mut rows: Vec<DynamicRow> = Vec::new();

                        for row in query_result {
                            let row = row.unwrap();
                            let mut dynamic_row = DynamicRow {
                                columns: std::collections::HashMap::new(),
                            };

                            for (i, column) in row.columns_ref().iter().enumerate() {
                                let column_name = column.name_str().to_string();
                                let mut column_value = None;
                                let row_opt: Option<Result<String, _>> = row.get_opt(i);
                                if let Some(Ok(value)) = row_opt {
                                    column_value = Some(value);
                                }
                                dynamic_row.columns.insert(column_name, column_value);
                            }
                            rows.push(dynamic_row);
                        }

                        result.rows = Some(rows);
                    }
                    Err(error) => {
                        result.error = Some(error.to_string());
                    }
                }

                result
            })
            .collect()
    }
}
