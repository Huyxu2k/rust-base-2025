use std::sync::Arc;

use async_trait::async_trait;
use axum::{body::Body, extract::{Request, State}, http::StatusCode, response::Response};

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
    async fn handle_request<B>(req: Request<B>, state: State<Arc<AppState>>) -> Result<Request<B>, Response>
    where 
        B:Send
    {
        let state = state.clone(); 
        let headers = req.headers();
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized"))
            .unwrap();

        if let Some(auth_header) = headers.get(AUTHORIZATION_HEADER) {
            let header = match auth_header.to_str() {
                Ok(h) => h,
                Err(_) => return Err(response),
            };

            if header.starts_with(BEARER) {
                if let Ok(is_valid) = state
                    .user_container
                    .security_service
                    .verify_token(TypeToken::Access, header.trim_start_matches(BEARER).to_string()).await
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
   async fn handle_request<B>(req: Request<B>, state: State<Arc<AppState>>) -> Result<Request<B>, Response>
   where 
        B:Send
    {
        // TODO
        Ok(req)
    }
}

// layer check role
#[derive(Debug, Clone)]
pub struct AuthorizationLayer;

#[async_trait]
impl THandler for AuthorizationLayer {
    async fn handle_request<B>(req: Request<B>, State(state): State<Arc<AppState>>) -> Result<Request<B>, Response> 
    where 
        B:Send
    {
        let role_header = req.headers().get("Role");

        let response = Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("Forbidden"))
            .unwrap();

        if let Some(role) = role_header {
            if let Ok(role_str) = role.to_str() {
                let role_owned = role_str.to_owned(); 
                if role_owned == "admin" {
                    return Ok(req);
                }
            }
        }
        Err(response)
    }
}

// layer save log
#[derive(Debug, Clone)]
pub struct LoggingLayer;

#[async_trait]
impl THandler for LoggingLayer {
    async fn handle_request<B>(req: Request<B>, State(state):State<Arc<AppState>>) -> Result<Request<B>, Response> 
    where 
        B:Send
    {
        // TODO
        Ok(req)
    }
}
