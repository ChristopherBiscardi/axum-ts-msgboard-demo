pub mod message;
pub mod shutdown;

use axum::{
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;
use tracing::instrument;

pub fn make_router() -> Router {
    let messages: Arc<Mutex<Vec<message::Message>>> =
        Arc::new(Mutex::new(vec![]));

    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/health", get(health))
        .route("/message", get(message::get))
        .route("/message", post(message::create))
        .layer(Extension(messages));

    let app = app.fallback(handler_404.into_service());

    app
}

#[instrument]
async fn health() -> &'static str {
    "ok"
}

#[instrument]
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "nothing to see here",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use serde_json::{json, Value};
    use std::net::{SocketAddr, TcpListener};
    use tower::ServiceExt; // for `app.oneshot()`

    #[tokio::test]
    async fn no_messages_is_empty_response() {
        let app = make_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/message")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body =
            hyper::body::to_bytes(response.into_body())
                .await
                .unwrap();
        let body: Value =
            serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({"data":[]}));
    }

    #[tokio::test]
    async fn insert_message() {
        let app = make_router();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/message")
                    .header(
                        http::header::CONTENT_TYPE,
                        mime::APPLICATION_JSON.as_ref(),
                    )
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                           "message": "hello world"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body =
            hyper::body::to_bytes(response.into_body())
                .await
                .unwrap();
        let body: Value =
            serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({"data":{"content":"hello world"}})
        );
    }

    #[tokio::test]
    async fn insert_and_retrieve_messages() {
        let listener = TcpListener::bind(
            "0.0.0.0:0".parse::<SocketAddr>().unwrap(),
        )
        .unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(make_router().into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        client
            .request(
                Request::builder()
                    .uri(format!("http://{}/message", addr))
                    .header(
                        http::header::CONTENT_TYPE,
                        mime::APPLICATION_JSON.as_ref(),
                    )
                    .method(http::Method::POST)
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                           "message": "hello world"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        client
            .request(
                Request::builder()
                    .uri(format!("http://{}/message", addr))
                    .method(http::Method::POST)
                    .header(
                        http::header::CONTENT_TYPE,
                        mime::APPLICATION_JSON.as_ref(),
                    )
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                           "message": "hello second"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        let response = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/message", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body =
            hyper::body::to_bytes(response.into_body())
                .await
                .unwrap();
        let body: Value =
            serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({"data":[{"content":"hello world"},{"content":"hello second"}]})
        );
    }
}
