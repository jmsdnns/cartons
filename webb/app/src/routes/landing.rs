use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::models::user::User;
use crate::AppState;

pub async fn home(State(state): State<AppState>, user: User) -> impl IntoResponse {
    println!("STATE: {:?}", state);
    println!("USER");
    println!("- id: {:?}", user.id);
    println!("- username: {:?}", user.username);

    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}
