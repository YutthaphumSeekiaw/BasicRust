use chrono::Utc;
use reqwest::Client;
use std::time::Duration;
use crate::models::StatusReport;

pub struct StatusReporter {
    client: Client,
    endpoint: String,
}

impl StatusReporter {
    pub fn new(endpoint: String, timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self { client, endpoint }
    }

    pub async fn report_status(
        &self,
        operation: &str,
        success: bool,
        details: Option<String>,
        order_id: Option<i32>,
    ) {
        let report = StatusReport {
            operation: operation.to_string(),
            success,
            timestamp: Utc::now(),
            details,
            order_id,
        };

        // Send status report in a non-blocking way
        let client = self.client.clone();
        let endpoint = self.endpoint.clone();
        
        tokio::spawn(async move {
            match client.post(&endpoint).json(&report).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        tracing::debug!("Status report sent successfully for operation: {}", operation);
                    } else {
                        tracing::warn!(
                            "Status report failed with status {}: {}",
                            response.status(),
                            operation
                        );
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to send status report for operation {}: {}", operation, e);
                }
            }
        });
    }

    pub async fn report_success(&self, operation: &str, order_id: Option<i32>) {
        self.report_status(operation, true, None, order_id).await;
    }

    pub async fn report_failure(&self, operation: &str, error: &str, order_id: Option<i32>) {
        self.report_status(operation, false, Some(error.to_string()), order_id).await;
    }
}