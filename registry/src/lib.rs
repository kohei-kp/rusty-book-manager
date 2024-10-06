use std::sync::Arc;

use adapter::{
    database::ConnectionPool,
    repository::{book::BookRepositoryImpl, health::HealthCheckRepositoryImpl},
};
use kernel::repository::{book::BookRepository, health::HealthCheckRepository};

// DIコンテナの役割を果たす構造体定義
#[derive(Clone)]
pub struct AppRegistry {
    pub health_check_repository: Arc<dyn HealthCheckRepository>,
    pub book_repository: Arc<dyn BookRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        // 依存解決
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));

        Self {
            health_check_repository,
            book_repository,
        }
    }

    // 依存解決したインスタンスを返すメソッド
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }
}
