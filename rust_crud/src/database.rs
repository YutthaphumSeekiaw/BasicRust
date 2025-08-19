use sqlx::{SqlitePool, Pool, Sqlite};
use std::time::Duration;
use crate::config::AppConfig;

pub type DatabasePool = Pool<Sqlite>;

pub async fn create_pool(config: &AppConfig) -> anyhow::Result<DatabasePool> {
    let pool = sqlx::SqlitePoolOptions::new()
        .max_connections(config.connection_pool_size)
        .acquire_timeout(config.request_timeout)
        .idle_timeout(Some(Duration::from_secs(300)))
        .connect(&config.database_url)
        .await?;

    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    tracing::info!("Database connection pool created successfully");
    Ok(pool)
}

pub async fn run_migrations(pool: &DatabasePool) -> anyhow::Result<()> {
    tracing::info!("Running database migrations");
    
    // Create orders table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            customer_name TEXT NOT NULL,
            product_name TEXT NOT NULL,
            quantity INTEGER NOT NULL CHECK (quantity > 0),
            unit_price REAL NOT NULL CHECK (unit_price > 0),
            total_amount REAL NOT NULL,
            order_date TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Pending',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            
            CHECK (status IN ('Pending', 'Processing', 'Shipped', 'Delivered', 'Cancelled'))
        );
        "#
    )
    .execute(pool)
    .await?;

    // Create indexes if they don't exist
    sqlx::query("CREATE INDEX IF NOT EXISTS IX_orders_customer_name ON orders(customer_name);")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS IX_orders_order_date ON orders(order_date);")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS IX_orders_status ON orders(status);")
        .execute(pool)
        .await?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}