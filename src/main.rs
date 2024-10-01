use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

// ハンドラ
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    // ルーターを作成
    let app = Router::new().route("/hello", get(hello_world));

    // localhost:8080 でリクエストを受け付ける
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    // 指定アドレスでリスナーを作成
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    // サーバーを起動
    axum::serve(listener, app).await.unwrap();
}
