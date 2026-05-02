use crate::presentation::http::health::health_check;
use axum::{Router, routing::get};

pub fn new() -> Router {
    Router::new().route("/health", get(health_check))
}

#[cfg(test)]
mod tests {
    use super::new;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn health_route_returns_ok_json() {
        let app = new();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("request should be built"),
            )
            .await
            .expect("router should return a response");

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        let json_body: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid JSON");

        assert_eq!(json_body, json!({ "status": "ok" }));
    }
}
