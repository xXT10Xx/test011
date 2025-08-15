use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{auth::verify_jwt, config::Config, database::Database};

pub async fn auth_middleware(
    State(database): State<Database>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    
    if path == "/health" || path.starts_with("/api/auth/") {
        return Ok(next.run(request).await);
    }

    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    let config = Config::from_env().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let claims = verify_jwt(token, &config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(claims.claims);

    Ok(next.run(request).await)
}