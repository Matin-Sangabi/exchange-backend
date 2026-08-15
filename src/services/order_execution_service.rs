use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::orders::Order, errors::AppError,
    repositories::order_execution::OrderExecutionRepository,
};

#[derive(Clone)]
pub struct OrderExecutionService {
    repository: Arc<dyn OrderExecutionRepository>,
}

impl OrderExecutionService {
    pub fn new(repository: Arc<dyn OrderExecutionRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute_order(&self, order_id: Uuid) -> Result<Order, AppError> {
        if order_id.is_nil() {
            return Err(AppError::OrderNotFound);
        }

        self.repository.execute(order_id).await
    }

    pub async fn cancel_order(&self, order_id: Uuid) -> Result<Order, AppError> {
        if order_id.is_nil() {
            return Err(AppError::OrderNotFound);
        }
        self.repository.cancel(order_id).await
    }
}
