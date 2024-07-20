use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::resource_path)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResourcePath {
    pub id: i32,
    pub title: String,
    pub classify: String,
    pub cover: String,
    pub url: String,
    pub introduction: String,
    pub type_: String,
    pub status: Option<i16>,
    pub remark : Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    
}