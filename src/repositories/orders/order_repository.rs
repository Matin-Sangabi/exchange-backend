use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::orders::Order, errors::AppError};

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn create(&self, order: &Order) -> Result<Order, AppError>;

    async fn find_by_id(&self, order_id: Uuid) -> Result<Option<Order>, AppError>;

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>, AppError>;

    async fn update(&self, order: &Order) -> Result<Order, AppError>;
}
