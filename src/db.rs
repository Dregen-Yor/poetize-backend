use crate::config::CFG;
use diesel::prelude::*;
pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&CFG.database.database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &CFG.database.database_url))
}
