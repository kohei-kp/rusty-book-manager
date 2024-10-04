use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

// DatabaseConfigからPgConnectionOptionsに変換する
fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

// sqlx::PgPoolをラップ
#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    // PgPoolへの参照を返す
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

// Postgres コネクションプールを作成
fn connect_database_with(cfg: DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(&cfg)))
}
