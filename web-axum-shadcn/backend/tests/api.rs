//! End-to-end tests for the HTTP API. Requests go through the real router
//! in memory (`tower::ServiceExt::oneshot`), so no port is opened.

use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode, header},
};
use backend::{AppState, app, cors_layer};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

async fn send(app: &Router, method: Method, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut request = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(json) => {
            request = request.header(header::CONTENT_TYPE, "application/json");
            Body::from(json.to_string())
        }
        None => Body::empty(),
    };

    let response = app
        .clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, json)
}

#[tokio::test]
async fn health_check_reports_ok() {
    let app = app(AppState::default());
    let (status, body) = send(&app, Method::GET, "/healthz", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "status": "ok" }));
}

#[tokio::test]
async fn notes_crud_round_trip() {
    let app = app(AppState::default());

    let (status, created) = send(
        &app,
        Method::POST,
        "/api/notes",
        Some(json!({ "title": "  First note  ", "content": "hello" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["title"], "First note", "title is trimmed");
    let id = created["id"].as_str().unwrap().to_string();

    let (status, list) = send(&app, Method::GET, "/api/notes", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (status, updated) = send(
        &app,
        Method::PUT,
        &format!("/api/notes/{id}"),
        Some(json!({ "title": "Renamed", "content": "changed" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["title"], "Renamed");
    assert_eq!(updated["content"], "changed");
    assert_eq!(updated["createdAt"], created["createdAt"]);

    let (status, fetched) = send(&app, Method::GET, &format!("/api/notes/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched, updated);

    let (status, _) = send(&app, Method::DELETE, &format!("/api/notes/{id}"), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, body) = send(&app, Method::GET, &format!("/api/notes/{id}"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["code"], "not_found");
}

#[tokio::test]
async fn list_is_newest_first() {
    let app = app(AppState::with_sample_notes());
    let (_, list) = send(&app, Method::GET, "/api/notes", None).await;
    let notes = list.as_array().unwrap();
    assert!(notes.len() >= 2);
    assert!(notes[0]["createdAt"].as_str() >= notes[1]["createdAt"].as_str());
}

#[tokio::test]
async fn blank_title_is_a_validation_error() {
    let app = app(AppState::default());
    let (status, body) = send(
        &app,
        Method::POST,
        "/api/notes",
        Some(json!({ "title": "   " })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["code"], "validation_error");
}

#[tokio::test]
async fn missing_title_is_a_validation_error() {
    let app = app(AppState::default());
    let (status, body) = send(
        &app,
        Method::POST,
        "/api/notes",
        Some(json!({ "content": "x" })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["code"], "validation_error");
}

#[tokio::test]
async fn malformed_json_is_a_bad_request() {
    let app = app(AppState::default());
    let request = Request::post("/api/notes")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{not json"))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["code"], "bad_request");
}

#[tokio::test]
async fn invalid_id_is_a_bad_request() {
    let app = app(AppState::default());
    let (status, body) = send(&app, Method::GET, "/api/notes/not-a-uuid", None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["code"], "bad_request");
}

#[tokio::test]
async fn unknown_note_is_not_found() {
    let app = app(AppState::default());
    let missing = "/api/notes/00000000-0000-0000-0000-000000000000";
    for method in [Method::GET, Method::DELETE] {
        let (status, body) = send(&app, method, missing, None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body["code"], "not_found");
    }
    let (status, _) = send(&app, Method::PUT, missing, Some(json!({ "title": "x" }))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn unknown_route_returns_json_404() {
    let app = app(AppState::default());
    let (status, body) = send(&app, Method::GET, "/nope", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["code"], "not_found");
}

#[tokio::test]
async fn cors_allows_only_configured_origins() {
    let app = app(AppState::default()).layer(cors_layer(&["http://localhost:1430".to_string()]));

    let request = |origin: &str| {
        Request::get("/api/notes")
            .header(header::ORIGIN, origin)
            .body(Body::empty())
            .unwrap()
    };

    let allowed = app
        .clone()
        .oneshot(request("http://localhost:1430"))
        .await
        .unwrap();
    assert_eq!(
        allowed
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .unwrap(),
        "http://localhost:1430"
    );

    let denied = app.oneshot(request("http://evil.example")).await.unwrap();
    assert!(
        denied
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_none()
    );
}
