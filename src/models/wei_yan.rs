use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::wei_yan)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WeiYan {
    pub id: i32,
    pub user_id: Option<i32>,
    pub like_count: Option<i32>,
    pub content: String,
    #[diesel(column_name = type_)]
    pub type_: String,
    pub source: Option<i32>,
    pub is_public: Option<i16>,
    pub create_time: Option<NaiveDateTime>,
}