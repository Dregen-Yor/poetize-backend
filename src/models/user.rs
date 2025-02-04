use diesel::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub username: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub user_status: Option<i16>,
    pub gender: Option<i16>,
    pub open_id: Option<String>,
    pub avatar: Option<String>,
    pub admire: Option<String>,
    pub subscribe: Option<String>,
    pub introduction: Option<String>,
    pub user_type: Option<i16>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub deleted: Option<i16>,
}
