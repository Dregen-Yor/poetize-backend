use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::family)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Family {
    pub id: i32,
    pub user_id: Option<i32>,
    pub bg_cover: String,
    pub man_cover: String,
    pub woman_cover: String,
    pub man_name: String,
    pub woman_name: String,
    pub timing: String,
    pub countdown_title: Option<String>,
    pub countdown_time: Option<String>,
    pub status: Option<i16>,
    pub family_info: Option<String>,
    pub like_count: Option<i32>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
}