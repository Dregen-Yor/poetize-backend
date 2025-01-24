use crate::config::CFG;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use once_cell::sync::OnceCell;
type PgPool = Pool<ConnectionManager<PgConnection>>;

static DB_POOL: OnceCell<PgPool> = OnceCell::new();
pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&CFG.database.database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &CFG.database.database_url))
}
