use crate::handlers::auth::Claims;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::sync::Arc;

/// 从请求中提取JWT token
pub fn extract_token_from_header(authorization: &str) -> Option<&str> {
    if authorization.starts_with("Bearer ") {
        Some(&authorization[7..])
    } else {
        None
    }
}

/// State containing JWT secret
pub struct JwtSecret(pub Arc<String>);

/// 验证JWT token的中间件
pub async fn auth_middleware(
    State(jwt_secret): State<Arc<String>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // 从请求头获取Authorization
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(auth) => extract_token_from_header(auth),
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let token = match token {
        Some(t) => t,
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 验证token
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            // 将claims存入请求扩展，供后续处理器使用
            let mut request = request;
            request.extensions_mut().insert(token_data.claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
