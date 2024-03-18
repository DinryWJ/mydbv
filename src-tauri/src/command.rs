use dashmap::DashMap;
use std::sync::Arc;

use crate::db_conn::thread_keeper::ThreadKeeper;

pub struct DbState {
    pub rw: Arc<DashMap<String, ThreadKeeper>>,
}
