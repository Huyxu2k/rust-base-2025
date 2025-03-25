use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{apps::axum::{error::ApiError, state::AppState}, modules::{auth::service::AuthService, user::repository::User}};


#[derive(Debug,Deserialize)]
pub struct LoginRequest{
    email_or_username: String,
    password: String
}

#[derive(Debug,Serialize)]
pub struct LoginResponse{
    token: String,
    user: User
}

pub struct AuthHandler;

impl AuthHandler {
    async fn login(Json(data):Json<LoginRequest>,service: State<Arc<dyn AuthService>>)->Result<Json<LoginResponse>,ApiError>{
        let (user,token)= service.login(&data.email_or_username,&data.password).await?;
        let rep= LoginResponse{ user, token};

        Ok(Json(rep))
    }
}

pub fn auth_router(state: State<Arc<AppState>>)->Router{
    Router::new()
             .route("/login",post(AuthHandler::login))
             //.route("/logout", method_router)
             .with_state(state.user_container.auth_service.clone())
 }