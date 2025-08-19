use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::Row;
use crate::database::DatabasePool;
use crate::models::{Order, CreateOrderRequest, UpdateOrderRequest, OrderStatus};

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Connection pool error: {0}")]
    Pool(String),
}

pub struct OrderRepository {
    pool: DatabasePool,
}

impl OrderRepository {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, request: CreateOrderRequest) -> Result<Order, RepositoryError> {
        let total_amount = request.unit_price * Decimal::from(request.quantity);
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            INSERT INTO orders (customer_name, product_name, quantity, unit_price, total_amount, order_date, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&request.customer_name)
        .bind(&request.product_name)
        .bind(request.quantity)
        .bind(request.unit_price)
        .bind(total_amount)
        .bind(now.to_rfc3339())
        .bind("Pending")
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid() as i32;

        Ok(Order {
            id,
            customer_name: request.customer_name,
            product_name: request.product_name,
            quantity: request.quantity,
            unit_price: request.unit_price,
            total_amount,
            order_date: now,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn find_all(&self) -> Result<Vec<Order>, RepositoryError> {
        let rows = sqlx::query_as::<_, Order>(
            "SELECT id, customer_name, product_name, quantity, unit_price, total_amount, order_date, status, created_at, updated_at FROM orders ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Order>, RepositoryError> {
        let row = sqlx::query_as::<_, Order>(
            "SELECT id, customer_name, product_name, quantity, unit_price, total_amount, order_date, status, created_at, updated_at FROM orders WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn update(&self, id: i32, request: UpdateOrderRequest) -> Result<Order, RepositoryError> {
        // First, get the current order
        let current = self.find_by_id(id).await?
            .ok_or_else(|| RepositoryError::Database(sqlx::Error::RowNotFound))?;

        // Build the updated values
        let customer_name = request.customer_name.unwrap_or(current.customer_name);
        let product_name = request.product_name.unwrap_or(current.product_name);
        let quantity = request.quantity.unwrap_or(current.quantity);
        let unit_price = request.unit_price.unwrap_or(current.unit_price);
        let status = request.status.unwrap_or(current.status);
        let total_amount = unit_price * Decimal::from(quantity);
        let now = Utc::now();

        let status_str = match status {
            OrderStatus::Pending => "Pending",
            OrderStatus::Processing => "Processing",
            OrderStatus::Shipped => "Shipped",
            OrderStatus::Delivered => "Delivered",
            OrderStatus::Cancelled => "Cancelled",
        };

        sqlx::query(
            r#"
            UPDATE orders 
            SET customer_name = ?, product_name = ?, quantity = ?,
                unit_price = ?, total_amount = ?, status = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&customer_name)
        .bind(&product_name)
        .bind(quantity)
        .bind(unit_price)
        .bind(total_amount)
        .bind(status_str)
        .bind(now.to_rfc3339())
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(Order {
            id,
            customer_name,
            product_name,
            quantity,
            unit_price,
            total_amount,
            order_date: current.order_date,
            status,
            created_at: current.created_at,
            updated_at: now,
        })
    }

    pub async fn delete(&self, id: i32) -> Result<bool, RepositoryError> {
        let result = sqlx::query("DELETE FROM orders WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}