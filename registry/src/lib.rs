use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;

// DIコンテナの役割を果たす構造体定義
#[derive(Clone)]
pub struct AppRegistry {
    pub health_check_repository: Arc<dyn HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        // 依存解決
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        Self {
            health_check_repository,
        }
    }

    // 依存解決したインスタンスを返すメソッド
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
}
