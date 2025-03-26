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
    pub async fn login(service: State<Arc<dyn AuthService>>,Json(data):Json<LoginRequest>)->Result<Json<LoginResponse>,ApiError>{
        let (user,token)= service.login(&data.email_or_username,&data.password).await?;
        let rep= LoginResponse{ user, token};

        Ok(Json(rep))
    }
}