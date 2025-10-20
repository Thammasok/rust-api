use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
struct AuthError {
    success: bool,
    message: String,
}

// Example authentication middleware
// In production, implement proper JWT validation
pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // Check for Authorization header
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // Check if it starts with "Bearer "
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                // TODO: Validate JWT token here
                // For now, just check if token is not empty
                if !token.is_empty() {
                    return Ok(next.run(request).await);
                }
            }
        }
    }

    // Return 401 Unauthorized if token is missing or invalid
    Err((
        StatusCode::UNAUTHORIZED,
        Json(AuthError {
            success: false,
            message: "Unauthorized - Missing or invalid token".to_string(),
        }),
    ))
}
