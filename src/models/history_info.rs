use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::history_info)]
pub struct HistoryInfo {
    pub id: i32,
    pub user_id: Option<i32>,
    pub ip: String,
    pub nation: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
}
