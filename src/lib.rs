//! # ztm-backend
//!
//! Library half of the crate: builds the application router so it can be
//! shared by `main.rs` and the integration tests in `tests/`.

use axum::Router;
use tower_http::trace::TraceLayer;

pub mod routes;

/// Builds the full application with all routes and middleware.
pub fn app() -> Router {
    routes::router().layer(TraceLayer::new_for_http())
}
