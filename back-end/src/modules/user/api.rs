use support_macro::{openapi};
use axum::{extract::{connect_info, State}, Json};
use crate::{utils::json_extractor::JsonExtractor, AppState};
use crate::utils::path_extractor::PathExtrator;

use super::model::{NewUser, User, UserRegister};
use super::service;


#[openapi(Get, "/users")]
pub async fn get_all_user(State(app_state): State<AppState>)->Result<Json<Vec<User>>,String>{
    let users=service::get_all_user(&app_state.pool).await.unwrap();

    Ok(Json(users))
}

#[openapi(Get, "/users/{id}")]
pub async fn get_user_by_id(State(app_state): State<AppState>,PathExtrator(id): PathExtrator<i32>)->Result<Json<User>,String>{
    let user=service::get_user_by_id(id, &app_state.pool).await.unwrap();
    Ok(Json(user))
}


#[openapi(Delete, "/users/{id}")]
pub async fn delete_user_by_id(State(app_state): State<AppState>,PathExtrator(id): PathExtrator<i32>)->Result<Json<i32>,String>{
    let id=service::delete_user(id, &app_state.pool).await.unwrap();
    Ok(Json(id))
}


#[openapi(Post, "/users")]
pub async fn create_new_user(State(app_state): State<AppState>,JsonExtractor(user_reg):JsonExtractor<UserRegister>)->Result<Json<User>,String>{
    //TODO
    let user=service::create_user(user_reg, &app_state.pool,0).await.unwrap();
    Ok(Json(user))
}
