# Design Document

## Overview

The Order CRUD API is a Rust-based REST service that provides complete CRUD operations for order management with SQL Server persistence and external status reporting. The system uses Axum as the web framework, SQLx for database operations, and Reqwest for HTTP client functionality to report operation status to external monitoring endpoints.

## Architecture

The system follows a layered architecture pattern:

```
┌─────────────────┐
│   REST API      │ ← Axum web server with route handlers
├─────────────────┤
│   Service Layer │ ← Business logic and orchestration
├─────────────────┤
│   Repository    │ ← Data access layer with SQLx
├─────────────────┤
│   SQL Server    │ ← Database persistence
└─────────────────┘

External Integration:
┌─────────────────┐
│ Status Reporter │ ← HTTP client for mock.com/api/process/status
└─────────────────┘
```

### Technology Stack

- **Web Framework**: Axum (async, performant, type-safe)
- **Database**: SQL Server with SQLx (compile-time checked queries)
- **HTTP Client**: Reqwest (async HTTP client for status reporting)
- **Serialization**: Serde (JSON handling)
- **Error Handling**: Anyhow + custom error types
- **Logging**: Tracing (structured logging)

## Components and Interfaces

### 1. Web Layer (Axum Handlers)

```rust
// Route handlers
async fn create_order(Json(order): Json<CreateOrderRequest>) -> Result<Json<Order>, ApiError>
async fn get_orders() -> Result<Json<Vec<Order>>, ApiError>
async fn get_order(Path(id): Path<i32>) -> Result<Json<Order>, ApiError>
async fn update_order(Path(id): Path<i32>, Json(order): Json<UpdateOrderRequest>) -> Result<Json<Order>, ApiError>
async fn delete_order(Path(id): Path<i32>) -> Result<StatusCode, ApiError>
```

### 2. Service Layer

```rust
pub struct OrderService {
    repository: Arc<OrderRepository>,
    status_reporter: Arc<StatusReporter>,
}

impl OrderService {
    async fn create_order(&self, request: CreateOrderRequest) -> Result<Order, ServiceError>
    async fn get_orders(&self) -> Result<Vec<Order>, ServiceError>
    async fn get_order(&self, id: i32) -> Result<Order, ServiceError>
    async fn update_order(&self, id: i32, request: UpdateOrderRequest) -> Result<Order, ServiceError>
    async fn delete_order(&self, id: i32) -> Result<(), ServiceError>
}
```

### 3. Repository Layer

```rust
pub struct OrderRepository {
    pool: SqlitePool,
}

impl OrderRepository {
    async fn create(&self, order: CreateOrderRequest) -> Result<Order, RepositoryError>
    async fn find_all(&self) -> Result<Vec<Order>, RepositoryError>
    async fn find_by_id(&self, id: i32) -> Result<Option<Order>, RepositoryError>
    async fn update(&self, id: i32, order: UpdateOrderRequest) -> Result<Order, RepositoryError>
    async fn delete(&self, id: i32) -> Result<bool, RepositoryError>
}
```

### 4. Status Reporter

```rust
pub struct StatusReporter {
    client: reqwest::Client,
    endpoint: String,
}

impl StatusReporter {
    async fn report_status(&self, operation: &str, success: bool, details: Option<String>)
}
```

## Data Models

### Order Entity

```rust
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

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}
```

### Request/Response DTOs

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(length(min = 1, max = 100))]
    pub customer_name: String,
    #[validate(length(min = 1, max = 100))]
    pub product_name: String,
    #[validate(range(min = 1))]
    pub quantity: i32,
    #[validate(range(min = 0.01))]
    pub unit_price: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderRequest {
    #[validate(length(min = 1, max = 100))]
    pub customer_name: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub product_name: Option<String>,
    #[validate(range(min = 1))]
    pub quantity: Option<i32>,
    #[validate(range(min = 0.01))]
    pub unit_price: Option<Decimal>,
    pub status: Option<OrderStatus>,
}
```

### Status Report Model

```rust
#[derive(Debug, Serialize)]
pub struct StatusReport {
    pub operation: String,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
    pub details: Option<String>,
    pub order_id: Option<i32>,
}
```

## Database Schema

```sql
CREATE TABLE orders (
    id INT IDENTITY(1,1) PRIMARY KEY,
    customer_name NVARCHAR(100) NOT NULL,
    product_name NVARCHAR(100) NOT NULL,
    quantity INT NOT NULL CHECK (quantity > 0),
    unit_price DECIMAL(10,2) NOT NULL CHECK (unit_price > 0),
    total_amount DECIMAL(10,2) NOT NULL,
    order_date DATETIME2 NOT NULL,
    status NVARCHAR(20) NOT NULL DEFAULT 'Pending',
    created_at DATETIME2 NOT NULL DEFAULT GETUTCDATE(),
    updated_at DATETIME2 NOT NULL DEFAULT GETUTCDATE(),
    
    CONSTRAINT CK_orders_status CHECK (status IN ('Pending', 'Processing', 'Shipped', 'Delivered', 'Cancelled'))
);

CREATE INDEX IX_orders_customer_name ON orders(customer_name);
CREATE INDEX IX_orders_order_date ON orders(order_date);
CREATE INDEX IX_orders_status ON orders(status);
```

## Error Handling

### Error Types Hierarchy

```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Order not found with id: {id}")]
    OrderNotFound { id: i32 },
    #[error("Status reporting failed: {0}")]
    StatusReporting(String),
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Connection pool error: {0}")]
    Pool(String),
}
```

### HTTP Status Code Mapping

- `200 OK`: Successful GET, PUT operations
- `201 Created`: Successful POST operations
- `204 No Content`: Successful DELETE operations
- `400 Bad Request`: Validation errors, malformed requests
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Database errors, unexpected failures
- `503 Service Unavailable`: Database connection failures

## Status Reporting Integration

### Report Timing
- **Success Reports**: Sent after successful database operations
- **Failure Reports**: Sent when operations fail at any stage
- **Non-blocking**: Status reporting failures don't affect main operation responses

### Report Format
```json
{
  "operation": "create_order|get_order|update_order|delete_order",
  "success": true|false,
  "timestamp": "2024-01-15T10:30:00Z",
  "details": "Optional error message or additional info",
  "order_id": 123
}
```

### Retry Strategy
- Single attempt for status reporting
- Log failures for monitoring
- Continue normal operation regardless of reporting status

## Testing Strategy

### Unit Tests
- **Repository Layer**: Test all CRUD operations with test database
- **Service Layer**: Test business logic with mocked dependencies
- **Status Reporter**: Test HTTP client with mock server
- **Error Handling**: Test all error scenarios and mappings

### Integration Tests
- **API Endpoints**: Test full request/response cycles
- **Database Integration**: Test with real SQL Server instance
- **Status Reporting**: Test with mock external API

### Test Database Setup
- Use SQL Server test containers for integration tests
- Separate test database with same schema
- Automated test data cleanup between tests

### Performance Tests
- Load testing for concurrent requests
- Database connection pool stress testing
- Status reporting under high load

## Configuration

### Environment Variables
```
DATABASE_URL=mssql://username:password@localhost/orderdb
STATUS_ENDPOINT=https://mock.com/api/process/status
SERVER_PORT=3000
LOG_LEVEL=info
CONNECTION_POOL_SIZE=10
REQUEST_TIMEOUT_SECONDS=30
```

### Application Configuration
```rust
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub status_endpoint: String,
    pub server_port: u16,
    pub connection_pool_size: u32,
    pub request_timeout: Duration,
}
```