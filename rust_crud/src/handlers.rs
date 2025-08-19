use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use crate::models::{Order, CreateOrderRequest, UpdateOrderRequest};
use crate::service::{OrderService, ServiceError};
use crate::errors::ApiError;

pub type AppState = Arc<OrderService>;

pub async fn create_order(
    State(service): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<Order>), ApiError> {
    let order = service.create_order(request).await?;
    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn get_orders(
    State(service): State<AppState>,
) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = service.get_orders().await?;
    Ok(Json(orders))
}

pub async fn get_order(
    State(service): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Order>, ApiError> {
    let order = service.get_order(id).await?;
    Ok(Json(order))
}

pub async fn update_order(
    State(service): State<AppState>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateOrderRequest>,
) -> Result<Json<Order>, ApiError> {
    let order = service.update_order(id, request).await?;
    Ok(Json(order))
}

pub async fn delete_order(
    State(service): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    service.delete_order(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn health_check() -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}