use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::{postgres::PgConnectOptions, PgPool};
use tokio::net::TcpListener;

// DB接続設定
struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

// アプリケーション用のDB設定構造体から、Postgres接続用の構造体に変換
impl From<DatabaseConfig> for PgConnectOptions {
    fn from(config: DatabaseConfig) -> Self {
        Self::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.username)
            .password(&config.password)
            .database(&config.database)
    }
}

// Postgres コネクションプールを作成
fn connect_database_with(config: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(config.into())
}

// リクエストがくると 200 OK を返す
async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_config = DatabaseConfig {
        host: "localhost".into(),
        port: 5432,
        username: "app".into(),
        password: "passwd".into(),
        database: "app".into(),
    };
    let pool = connect_database_with(database_config);

    // ルーターを作成
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        .with_state(pool);

    // localhost:8080 でリクエストを受け付ける
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    // 指定アドレスでリスナーを作成
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    // サーバーを起動
    Ok(axum::serve(listener, app).await?)
}

#[tokio::test]
async fn health_check_workds() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

#[sqlx::test]
async fn health_check_db_works(pool: sqlx::PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}
