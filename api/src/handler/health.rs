use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

// リクエストがくると 200 OK を返す
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

// Stateに登録されているAooRegistoryを取り出す
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
