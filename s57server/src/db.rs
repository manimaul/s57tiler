use std::env;

use crate::errors::ErrMapper;
use actix_web;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

lazy_static! {
    static ref POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url.as_str());
    Pool::new(manager).expect(&format!("Error connecting to {}", database_url))
}

pub fn db_conn() -> Result<PooledConnection<ConnectionManager<PgConnection>>, actix_web::Error> {
    POOL.get()
        .map_internal_server_error("error getting db connection")
}