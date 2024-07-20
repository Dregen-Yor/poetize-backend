use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::comment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub source: i32,
    pub type_: String,
    pub parent_comment_id: Option<i32>,
    pub user_id: Option<i32>,
    pub floor_comment_id: Option<i32>,
    pub parent_user_id: Option<i32>,
    pub like_count: Option<i32>,
    pub comment_content: String,
    pub comment_info: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
}