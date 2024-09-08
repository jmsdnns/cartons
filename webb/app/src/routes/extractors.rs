use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use axum_macros::FromRequest;

use crate::auth::extract_token;
use crate::errors::AppError;
use crate::models::user::User;
use crate::AppState;

// JSON

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct JsonExtractor<T>(pub T);

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::BodyParsingError(rejection.to_string())
    }
}

//JWT Token

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        else {
            return Err(AppError::Unauthorized);
        };

        let Ok(claims) = extract_token(&state.hmac_key, bearer.token()) else {
            return Err(AppError::Unauthorized);
        };

        let Ok(user) = User::load(&state.db, &claims.username).await else {
            return Err(AppError::Unauthorized);
        };

        Ok(user)
    }
}
