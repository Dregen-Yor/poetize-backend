use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::article)]
pub struct Article {
    pub id: i32,
    pub user_id: Option<i32>,
    pub sort_id: Option<i32>,
    pub label_id: Option<i32>,
    pub article_cover: Option<String>,
    pub article_title: String,
    pub article_content: String,
    pub video_url: Option<String>,
    pub view_count: Option<i32>,
    pub like_count: Option<i32>,
    pub view_status: Option<i16>,
    pub password: Option<String>,
    pub tips: Option<String>,
    pub recommend_status: Option<i16>,
    pub comment_status: Option<i16>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub deleted: Option<i16>,
}