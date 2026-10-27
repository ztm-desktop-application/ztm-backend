use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    version: &'static str,
}

/// Server version as `YYYYMMDD.N.0`: the UTC date of the last merge to main
/// that changed courses, with N counting earlier such merges that day, or
/// `0.0.0` before any. Computed in `build.rs`.
pub async fn version() -> Json<Version> {
    Json(Version {
        version: env!("APP_VERSION"),
    })
}
