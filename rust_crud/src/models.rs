use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: i32,
    pub customer_name: String,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total_amount: Decimal,
    pub order_date: DateTime<Utc>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "PascalCase")]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Pending
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(length(min = 1, max = 100, message = "Customer name must be between 1 and 100 characters"))]
    pub customer_name: String,
    
    #[validate(length(min = 1, max = 100, message = "Product name must be between 1 and 100 characters"))]
    pub product_name: String,
    
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,
    
    #[validate(range(min = 0.01, message = "Unit price must be greater than 0"))]
    pub unit_price: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderRequest {
    #[validate(length(min = 1, max = 100, message = "Customer name must be between 1 and 100 characters"))]
    pub customer_name: Option<String>,
    
    #[validate(length(min = 1, max = 100, message = "Product name must be between 1 and 100 characters"))]
    pub product_name: Option<String>,
    
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: Option<i32>,
    
    #[validate(range(min = 0.01, message = "Unit price must be greater than 0"))]
    pub unit_price: Option<Decimal>,
    
    pub status: Option<OrderStatus>,
}

#[derive(Debug, Serialize)]
pub struct StatusReport {
    pub operation: String,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
    pub details: Option<String>,
    pub order_id: Option<i32>,
}