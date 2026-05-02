use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug)]
struct HealthService;

impl HealthService {
    fn is_healthy(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub async fn health_check() -> impl IntoResponse {
    let health_service = HealthService;

    if health_service.is_healthy() {
        (StatusCode::OK, Json(HealthResponse { status: "ok" }))
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "unhealthy",
            }),
        )
    }
}
