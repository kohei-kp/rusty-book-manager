use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

// リクエストがくると 200 OK を返す
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<()> {
    // ルーターを作成
    let app = Router::new().route("/health", get(health_check));

    // localhost:8080 でリクエストを受け付ける
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    // 指定アドレスでリスナーを作成
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    // サーバーを起動
    Ok(axum::serve(listener, app).await?)
}
