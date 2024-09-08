use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
//use chrono::{Duration, Utc};
//use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::auth::{create_token, hash_password, verify_password};
use crate::errors::AppError;
use crate::models::user::User;
//use crate::routes::extractors::{Claims, JsonExtractor};
use crate::routes::extractors::JsonExtractor;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateInput {
    username: String,
    password: String,
}

pub async fn create(
    State(state): State<AppState>,
    JsonExtractor(input): JsonExtractor<CreateInput>,
) -> impl IntoResponse {
    let Ok(password_hash) = hash_password(input.password).await else {
        return AppError::InternalServerError.into_response();
    };

    match User::create(&state.db, &input.username, &password_hash).await {
        Ok(_) => (StatusCode::OK, Json(json!({"status": "ok"}))).into_response(),
        Err(e) => AppError::Conflict(e.to_string()).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginOutput {
    access_token: String,
    token_type: String,
}

pub async fn login(
    State(state): State<AppState>,
    JsonExtractor(input): JsonExtractor<LoginInput>,
) -> impl IntoResponse {
    let Ok(user) = User::load(&state.db, &input.username).await else {
        return AppError::Unauthorized.into_response();
    };

    let Ok(_) = verify_password(input.password, user.password_hash).await else {
        return AppError::Unauthorized.into_response();
    };

    let Ok(token) = create_token(&state.hmac_key, &input.username) else {
        return AppError::Unauthorized.into_response();
    };

    let lo = LoginOutput {
        access_token: token,
        token_type: "Bearer".to_string(),
    };

    (StatusCode::OK, Json(lo)).into_response()
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/create", post(create))
        .route("/login", post(login))
        .with_state(state)
}
