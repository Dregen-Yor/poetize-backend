use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Article model
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::sys_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SysConfig{
    pub id:i32,
    pub config_name:String,
    pub config_key:String,
    pub config_value: Option<String>,
    pub config_type: Option<String>,
}