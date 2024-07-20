use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::tree_hole)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TreeHole {
    pub id: i32,
    pub avatar: Option<String>,
    pub message: String,
    pub create_time: Option<chrono::NaiveDateTime>
}