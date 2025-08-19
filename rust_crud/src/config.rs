use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub status_endpoint: String,
    pub server_port: u16,
    pub connection_pool_size: u32,
    pub request_timeout: Duration,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok(); // Load .env file if it exists
        
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:orders.db".to_string()),
            status_endpoint: std::env::var("STATUS_ENDPOINT")
                .unwrap_or_else(|_| "https://mock.com/api/process/status".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            connection_pool_size: std::env::var("CONNECTION_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            request_timeout: Duration::from_secs(
                std::env::var("REQUEST_TIMEOUT_SECONDS")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()?
            ),
        })
    }
}