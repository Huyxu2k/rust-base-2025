use axum::{
    body::Body, extract::Request, http::StatusCode, response::Response
};

use crate::{apps::axum::state::AppState, modules::auth::security::TypeToken};
use super::{THandler, AUTHORIZATION_HEADER, BEARER};

pub async fn health_check()->Response{
    let response = Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Success"))
            .unwrap();

    response
}

// layer check access token
#[derive(Debug,Clone)]
pub struct AccessTokenLayer;

impl THandler for AccessTokenLayer {
    fn handle_request<B>(req: Request<B>, state: AppState) -> Result<Request<B>, Response> {
        let headers= req.headers();
        let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Unauthorized"))
                .unwrap();

        if let Some(auth_header)=headers.get(AUTHORIZATION_HEADER){
           let header = auth_header.to_str().unwrap_or("");
           if header.starts_with(BEARER) && verify_token(header.to_string(),TypeToken::Access,&state).unwrap(){
             return Ok(req);
           }
           Err(response)
        }else {
            Err(response)
        }
    }
}


// layer check refresh token
#[derive(Debug,Clone)]
pub struct RefreshTokenLayer;

impl THandler for RefreshTokenLayer {
    fn handle_request<B>(req: Request<B>, state: AppState) -> Result<Request<B>, Response> {
        // TODO
        Ok(req)
    }
}

// layer check role
#[derive(Debug,Clone)]
pub struct AuthorizationLayer;

impl THandler for AuthorizationLayer {
    fn handle_request<B>(req: Request<B>, state: AppState) -> Result<Request<B>, Response> {
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
#[derive(Debug,Clone)]
pub struct LoggingLayer;

impl THandler for LoggingLayer {
    fn handle_request<B>(req: Request<B>, state: AppState) -> Result<Request<B>, Response> {
        // TODO
        Ok(req)
    }
}

