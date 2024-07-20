use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::resource)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Resource {
    pub id: i32,
    pub user_id: Option<i32>,
    pub type_: String,
    pub path: String,
    pub size: Option<i32>,
    pub original_name: String,
    pub mime_type: String,
    pub status: Option<i16>,
    pub store_type:String,
    pub create_time: Option<chrono::NaiveDateTime>,
}