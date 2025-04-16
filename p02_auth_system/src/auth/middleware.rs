use crate::auth::jwt::{Claims, validate_jwt};
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{Authorization, authorization::Bearer},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use std::env;

pub struct AuthToken(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Missing or invalid Authorization header",
                    )
                        .into_response()
                })?;

        // Validate the token (use your secret)
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        match validate_jwt(&bearer.token()) {
            Ok(claims) => Ok(AuthToken(claims)),
            Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token").into_response()),
        }
    }
}
