use std::env;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2_postgres::postgres::Config;

use crate::errors::{ErrMapper, Result};

struct DbConfig {
    user: String,
    pass: String,
    host: String,
    name: String
}

impl DbConfig {
    fn url(&self) -> String {
        format!("postgres://{}:{}@{}/{}", self.user, self.pass, self.host, self.name)
    }

    fn config(&self) -> Config {
        let mut config = Config::new();
        config.user(&self.user);
        config.host(&self.host);
        config.password(&self.pass);
        config.host(&self.host);
        config.dbname(&self.name);
        config
    }
}

lazy_static! {
    static ref POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

lazy_static! {
    static ref POOL2: Pool<PostgresConnectionManager<NoTls>> = create_db_pool2();
}

lazy_static! {
    static ref DB_CONFIG: DbConfig = {
        dotenv().ok();
        DbConfig {
            user: env::var("DB_USER").expect("DB_USER must be set"),
            pass: env::var("DB_PASS").expect("DB_PASS must be set"),
            host: env::var("DB_HOST").expect("DB_HOST must be set"),
            name: env::var("DB_NAME").expect("DB_NAME must be set"),
        }
    };
}

fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(DB_CONFIG.url());
    Pool::new(manager).expect(&format!("Error connecting to: {}", DB_CONFIG.url()))
}

fn create_db_pool2() -> Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new(
        DB_CONFIG.config(),
        NoTls,
    );
    r2d2::Pool::new(manager).expect(&format!("Error connecting to: {}", DB_CONFIG.url()))
}

pub fn db_conn() -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
    POOL.get()
        .map_internal_server_error("error getting db connection")
}

pub fn db_conn2() -> Result<PooledConnection<PostgresConnectionManager<NoTls>>> {
    POOL2.get()
        .map_internal_server_error("error getting db connection")
}