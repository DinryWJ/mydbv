use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct DynamicRow {
    pub columns: std::collections::HashMap<String, Option<String>>,
}
