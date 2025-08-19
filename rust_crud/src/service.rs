use std::sync::Arc;
use validator::Validate;
use crate::models::{Order, CreateOrderRequest, UpdateOrderRequest};
use crate::repository::{OrderRepository, RepositoryError};
use crate::status_reporter::StatusReporter;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Order not found with id: {id}")]
    OrderNotFound { id: i32 },
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Status reporting failed: {0}")]
    StatusReporting(String),
}

pub struct OrderService {
    repository: Arc<OrderRepository>,
    status_reporter: Arc<StatusReporter>,
}

impl OrderService {
    pub fn new(repository: Arc<OrderRepository>, status_reporter: Arc<StatusReporter>) -> Self {
        Self {
            repository,
            status_reporter,
        }
    }

    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<Order, ServiceError> {
        // Validate the request
        if let Err(validation_errors) = request.validate() {
            let error_msg = format!("Validation failed: {:?}", validation_errors);
            self.status_reporter
                .report_failure("create_order", &error_msg, None)
                .await;
            return Err(ServiceError::Validation(error_msg));
        }

        match self.repository.create(request).await {
            Ok(order) => {
                self.status_reporter
                    .report_success("create_order", Some(order.id))
                    .await;
                Ok(order)
            }
            Err(e) => {
                let error_msg = format!("Failed to create order: {}", e);
                self.status_reporter
                    .report_failure("create_order", &error_msg, None)
                    .await;
                Err(ServiceError::Repository(e))
            }
        }
    }

    pub async fn get_orders(&self) -> Result<Vec<Order>, ServiceError> {
        match self.repository.find_all().await {
            Ok(orders) => {
                self.status_reporter
                    .report_success("get_orders", None)
                    .await;
                Ok(orders)
            }
            Err(e) => {
                let error_msg = format!("Failed to get orders: {}", e);
                self.status_reporter
                    .report_failure("get_orders", &error_msg, None)
                    .await;
                Err(ServiceError::Repository(e))
            }
        }
    }

    pub async fn get_order(&self, id: i32) -> Result<Order, ServiceError> {
        match self.repository.find_by_id(id).await {
            Ok(Some(order)) => {
                self.status_reporter
                    .report_success("get_order", Some(id))
                    .await;
                Ok(order)
            }
            Ok(None) => {
                let error_msg = format!("Order not found with id: {}", id);
                self.status_reporter
                    .report_failure("get_order", &error_msg, Some(id))
                    .await;
                Err(ServiceError::OrderNotFound { id })
            }
            Err(e) => {
                let error_msg = format!("Failed to get order {}: {}", id, e);
                self.status_reporter
                    .report_failure("get_order", &error_msg, Some(id))
                    .await;
                Err(ServiceError::Repository(e))
            }
        }
    }

    pub async fn update_order(&self, id: i32, request: UpdateOrderRequest) -> Result<Order, ServiceError> {
        // Validate the request
        if let Err(validation_errors) = request.validate() {
            let error_msg = format!("Validation failed: {:?}", validation_errors);
            self.status_reporter
                .report_failure("update_order", &error_msg, Some(id))
                .await;
            return Err(ServiceError::Validation(error_msg));
        }

        match self.repository.update(id, request).await {
            Ok(order) => {
                self.status_reporter
                    .report_success("update_order", Some(id))
                    .await;
                Ok(order)
            }
            Err(RepositoryError::Database(sqlx::Error::RowNotFound)) => {
                let error_msg = format!("Order not found with id: {}", id);
                self.status_reporter
                    .report_failure("update_order", &error_msg, Some(id))
                    .await;
                Err(ServiceError::OrderNotFound { id })
            }
            Err(e) => {
                let error_msg = format!("Failed to update order {}: {}", id, e);
                self.status_reporter
                    .report_failure("update_order", &error_msg, Some(id))
                    .await;
                Err(ServiceError::Repository(e))
            }
        }
    }

    pub async fn delete_order(&self, id: i32) -> Result<(), ServiceError> {
        match self.repository.delete(id).await {
            Ok(true) => {
                self.status_reporter
                    .report_success("delete_order", Some(id))
                    .await;
                Ok(())
            }
            Ok(false) => {
                let error_msg = format!("Order not found with id: {}", id);
                self.status_reporter
                    .report_failure("delete_order", &error_msg, Some(id))
                    .await;
                Err(ServiceError::OrderNotFound { id })
            }
            Err(e) => {
                let error_msg = format!("Failed to delete order {}: {}", id, e);
                self.status_reporter
                    .report_failure("delete_order", &error_msg, Some(id))
                    .await;
                Err(ServiceError::Repository(e))
            }
        }
    }
}