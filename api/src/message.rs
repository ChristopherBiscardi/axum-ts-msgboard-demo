use std::sync::{Arc, Mutex};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};
use valuable::Valuable;

#[derive(Clone, Serialize)]
pub struct Message {
    content: String,
}

#[derive(Debug, Valuable, Deserialize)]
pub struct CreateMessage {
    message: String,
}
#[instrument(skip(messages_storage))]
pub async fn create(
    Json(payload): Json<CreateMessage>,
    Extension(messages_storage): Extension<
        Arc<Mutex<Vec<Message>>>,
    >,
) -> impl IntoResponse {
    match messages_storage.lock() {
        Ok(mut messages) => {
            info!("adding message");
            let message = Message {
                content: payload.message,
            };
            messages.push(message.clone());
            ApiResponse::Created { data: message }
        }
        Err(e) => {
            error!(
                error = e.to_string().as_str(),
                "messages_storage lock failed"
            );
            ApiResponse::Error {
                error: String::from(
                    "internal server error",
                ),
            }
        }
    }
}

#[instrument(skip(messages_storage))]
pub async fn get(
    Extension(messages_storage): Extension<
        Arc<Mutex<Vec<Message>>>,
    >,
) -> impl IntoResponse {
    match messages_storage.lock() {
        Ok(messages) => ApiResponse::Success {
            data: messages.clone(),
        },
        Err(e) => {
            error!(
                error = e.to_string().as_str(),
                "messages_storage lock failed"
            );

            ApiResponse::Error {
                error: String::from(
                    "internal server error",
                ),
            }
        }
    }
}

#[derive(Serialize)]
#[serde(untagged, rename_all = "camelCase")]
enum ApiResponse<T: Serialize> {
    Success { data: T },
    Created { data: T },
    Error { error: String },
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Success { data: _ } => (
                StatusCode::OK,
                serde_json::to_string(&self).unwrap(),
            )
                .into_response(),
            ApiResponse::Created { data: _ } => (
                StatusCode::CREATED,
                serde_json::to_string(&self).unwrap(),
            )
                .into_response(),
            ApiResponse::Error { error: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::to_string(&self).unwrap(),
            )
                .into_response(),
        }
    }
}
