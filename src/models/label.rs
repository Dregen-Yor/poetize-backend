use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::label)]
pub struct Label {
    pub id: i32,
    pub sort_id: Option<i32>,
    pub label_name: String,
    pub label_description: Option<String>,
}