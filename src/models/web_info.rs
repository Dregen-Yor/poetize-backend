use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::web_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WebInfo{
    pub id:i32,
    pub web_name:String,
    pub web_title: String,
    pub notices: String,
    pub footer:String,
    pub background_image:Option<String>,
    pub avatar:String,
    pub random_avatar:Option<String>,
    pub random_name :Option<String>,
    pub random_cover:Option<String>,
    pub waifu_json:Option<String>,
    pub status:Option<i16>,
}