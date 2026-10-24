use axum::http::StatusCode;

/// Liveness probe for load balancers and monitoring.
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
