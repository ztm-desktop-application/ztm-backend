use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn version_returns_date_based_semver() {
    let response = ztm_backend::app()
        .oneshot(
            Request::builder()
                .uri("/version")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let version = json["version"].as_str().unwrap();

    let parts: Vec<&str> = version.split('.').collect();
    assert_eq!(parts.len(), 3, "expected MAJOR.MINOR.PATCH, got {version}");
    assert!(
        parts.iter().all(|p| p.parse::<u64>().is_ok()),
        "expected numeric parts, got {version}"
    );
    assert_eq!(parts[2], "0", "expected patch 0, got {version}");
    // 0.0.0 until the first courses change is committed.
    if version != "0.0.0" {
        assert_eq!(parts[0].len(), 8, "expected YYYYMMDD major, got {version}");
    }
}
