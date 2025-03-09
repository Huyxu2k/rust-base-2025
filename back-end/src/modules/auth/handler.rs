use axum::extract::Query;
use support_macro::openapi;
use axum::{extract::State, Json};
use crate::AppState;
use super::model::UserLogin;

use super::{repository, Token};

#[openapi(Get, "/refresh_token")]
pub async fn create_token(State(app_state): State<AppState>)->Result<Json<Token>,String>{
    todo!()
}


#[openapi(Get, "/login")]
pub async fn login(Query(user_login):Query<UserLogin>,State(app_state): State<AppState>)->Result<Json<String>,String>{
    //Find user by user or email
    
    todo!()
}

#[openapi(Get, "/logout")]
pub async fn logout(State(app_state): State<AppState>)->Result<Json<String>,String>{
    todo!()
}
