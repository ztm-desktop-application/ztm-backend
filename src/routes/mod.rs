//! Route table. Each endpoint lives in its own module.

use axum::{Router, routing::get};

mod health_check;

pub fn router() -> Router {
    Router::new().route("/health-check", get(health_check::health_check))
}
