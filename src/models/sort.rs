use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::sort)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sort {
    pub id :i32,
    pub sort_name:String,
    pub sort_description:String,
    pub sort_type: Option<i16>,
    pub priority: Option<i32>,
}