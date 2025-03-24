use std::sync::Arc;

use async_trait::async_trait;
use axum::{body::Body, extract::Request, http::StatusCode, response::Response};

use super::{THandler, AUTHORIZATION_HEADER, BEARER};
use crate::{apps::axum::state::AppState, modules::auth::security::TypeToken};

pub async fn health_check() -> Response {
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Success"))
        .unwrap();

    response
}

// layer check access token
#[derive(Debug, Clone)]
pub struct AccessTokenLayer;

#[async_trait]
impl THandler for AccessTokenLayer {
    async fn handle_request<B>(req: Request<B>, state: Arc<AppState>) -> Result<Request<B>, Response> {
        let headers = req.headers();
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized"))
            .unwrap();

        if let Some(auth_header) = headers.get(AUTHORIZATION_HEADER) {
            let header = match auth_header.to_str() {
                Ok(h) => h,
                Err(_) => return Err(response), // Trả về lỗi nếu không thể parse header
            };

            if header.starts_with(BEARER) {
                if let Ok(is_valid) = state
                    .user_container
                    .security_service
                    .verify_token(TypeToken::Access, header.to_string()).await
                {
                    if is_valid {
                        return Ok(req);
                    }
                }
            }
        }
        Err(response)
    }
}

// layer check refresh token
#[derive(Debug, Clone)]
pub struct RefreshTokenLayer;

#[async_trait]
impl THandler for RefreshTokenLayer {
    fn handle_request<B>(req: Request<B>, state: Arc<AppState>) -> Result<Request<B>, Response> {
        // TODO
        Ok(req)
    }
}

// layer check role
#[derive(Debug, Clone)]
pub struct AuthorizationLayer;

#[async_trait]
impl THandler for AuthorizationLayer {
    fn handle_request<B>(req: Request<B>, state: Arc<AppState>) -> Result<Request<B>, Response> {
        let role_header = req.headers().get("Role");
        let response = Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("Forbidden"))
            .unwrap();

        if let Some(role) = role_header {
            //TODO Check role
            if role == "admin" {
                return Ok(req);
            }
        }
        return Err(response);
    }
}

// layer save log
#[derive(Debug, Clone)]
pub struct LoggingLayer;

impl THandler for LoggingLayer {
    fn handle_request<B>(req: Request<B>, state: Arc<AppState>) -> Result<Request<B>, Response> {
        // TODO
        Ok(req)
    }
}
