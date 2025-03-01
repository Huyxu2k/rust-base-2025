use async_trait::async_trait;
use support_macro::openapi;
use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};
use crate::{utils::extractor_body::JsonExtractor, AppState};
use crate::utils::extractor_path::PathExtrator;

use super::model::{User, UserRegister};
use super::repository;


#[openapi(Get, "/users")]
pub async fn get_all_user(State(app_state): State<AppState>)->Result<Json<Vec<User>>,String>{
    let users=repository::get_all_user(&app_state.pool).await.unwrap();

    Ok(Json(users))
}

#[openapi(Get, "/users/{id}")]
pub async fn get_user_by_id(State(app_state): State<AppState>,PathExtrator(id): PathExtrator<i32>)->Result<Json<User>,String>{
    let user=repository::get_user_by_id(id, &app_state.pool).await.unwrap();
    Ok(Json(user))
}


#[openapi(Delete, "/users/{id}")]
pub async fn delete_user_by_id(State(app_state): State<AppState>,PathExtrator(id): PathExtrator<i32>)->Result<Json<i32>,String>{
    let id=repository::delete_user(id, &app_state.pool).await.unwrap();
    Ok(Json(id))
}


#[openapi(Post, "/users")]
pub async fn create_new_user(State(app_state): State<AppState>,user: Query<String>)->Result<Json<User>,String>{
    //TODO
    // let user=service::create_user(user_reg, &app_state.pool,0).await.unwrap();
    // Ok(Json(user))
    todo!()
}

//update user api use trait

use crate::modules::traits::{params_body::BodyParamsApi,params_query::QueryParamsApi,params_path::PathParamsApi};

pub struct UserHandler;

#[async_trait]
impl PathParamsApi<i32> for UserHandler{
    async fn get_by_id(Path(user_id): Path<i32>, State(app_state): State<AppState>)-> impl IntoResponse{
        format!("Get Success :{}",user_id)
    }
    async fn delete_by_id(Path(user_id): Path<i32>, State(app_state): State<AppState>)-> impl IntoResponse{
        format!("Delete Success:{}",user_id)
    }
}



