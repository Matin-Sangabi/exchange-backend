use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::orders::Order, errors::AppError};

#[async_trait]
pub trait OrderExecutionRepository: Send + Sync {
    async fn execute(&self, order_id: Uuid) -> Result<Order, AppError>;
}
