use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::orders::Order, errors::AppError, services::order_service::{OrderFilter, OrderStats, },
};

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn create(&self, order: &Order) -> Result<Order, AppError>;

    async fn find_by_id(&self, order_id: Uuid) -> Result<Option<Order>, AppError>;

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
        filter: &OrderFilter,
    ) -> Result<Vec<Order>, AppError>;

    async fn update(&self, order: &Order) -> Result<Order, AppError>;

    async fn count_by_user_id(&self, user_id: Uuid, filter: &OrderFilter) -> Result<i64, AppError>;

    async fn get_user_stats(&self, user_id: Uuid) -> Result<OrderStats, AppError>;
}
