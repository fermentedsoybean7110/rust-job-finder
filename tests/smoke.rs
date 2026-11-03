use std::net::SocketAddr;

use axum::{
    Json, Router,
    body::Body,
    extract::Query,
    http::{Request, StatusCode},
    routing::get,
};
use http_body_util::BodyExt;
use rust_job_finder::{api::JobsClient, app};
use serde_json::Value;
use std::collections::HashMap;
use tower::ServiceExt;

async fn spawn_mock(router: Router) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    format!("http://{addr}")
}

async fn get_page(base: String) -> (StatusCode, String) {
    let res = app(JobsClient::new(base))
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let body = res.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8_lossy(&body).into_owned())
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/jobs.json")).unwrap()
}

#[tokio::test]
async fn index_lists_jobs_from_api() {
    let mock = Router::new().route(
        "/jobs",
        get(|Query(q): Query<HashMap<String, String>>| async move {
            assert_eq!(q.get("limit").map(String::as_str), Some("20"));
            Json(fixture())
        }),
    );
    let (status, body) = get_page(spawn_mock(mock).await).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("geechs 掲載案件"));
    assert!(body.contains("月額 800,000〜1,000,000円"));
    assert!(body.contains("/jobs/cmvgblyxf18jnkyv3eqke6i0w"));
    assert!(body.contains("https://geechs-job.com/project/details/12323"));
}

#[tokio::test]
async fn index_shows_error_page_on_upstream_500() {
    let mock = Router::new().route("/jobs", get(|| async { StatusCode::INTERNAL_SERVER_ERROR }));
    let (status, body) = get_page(spawn_mock(mock).await).await;
    assert_eq!(status, StatusCode::BAD_GATEWAY);
    assert!(body.contains("エラーが発生しました"));
}

#[tokio::test]
async fn index_shows_error_page_on_invalid_json() {
    let mock = Router::new().route("/jobs", get(|| async { "not json" }));
    let (status, _) = get_page(spawn_mock(mock).await).await;
    assert_eq!(status, StatusCode::BAD_GATEWAY);
}

#[tokio::test]
async fn index_shows_error_page_when_api_unreachable() {
    let (status, _) = get_page("http://127.0.0.1:1".to_string()).await;
    assert_eq!(status, StatusCode::BAD_GATEWAY);
}

#[tokio::test]
async fn client_maps_404_to_not_found() {
    let mock = Router::new().route(
        "/jobs/{id}",
        get(|| async {
            (
                StatusCode::NOT_FOUND,
                "{\"error\":{\"code\":\"not_found\"}}",
            )
        }),
    );
    let client = JobsClient::new(spawn_mock(mock).await);
    let err = client.get_job("missing").await.unwrap_err();
    assert!(matches!(err, rust_job_finder::api::ApiError::NotFound));
}
