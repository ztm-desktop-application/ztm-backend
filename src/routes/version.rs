use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    version: &'static str,
}

/// Server version as `YYYYMMDD.N.0`: the UTC date of the last merge to main,
/// with N counting earlier merges that day. Computed in `build.rs`.
pub async fn version() -> Json<Version> {
    Json(Version {
        version: env!("APP_VERSION"),
    })
}
