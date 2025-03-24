use std::sync::Arc;

use axum::routing::{delete, get};
use axum::{Extension, Router};
use crate::apps::axum::error::ApiError;
use crate::apps::axum::state::AppState;
use crate::modules::user::service::UserService;
use crate::modules::PaginationRequest;
use support_macro::openapi;
use axum::{extract::{Path, State}, Json};

use crate::modules::user::repository::{CreateUserRequest, FilterUsersRequest, User, UserIdentity};

//Implement api trait for UserHandler 
pub struct UserHandler;

impl UserHandler {
    pub async fn get_users(user_service: State<Arc<dyn UserService>>,Extension(identity): Extension<UserIdentity>)->Result<Json<Vec<User>>,ApiError>{
        let filter=FilterUsersRequest{
            name: None,
            pagination: PaginationRequest::default() ,
        };
        let users=user_service.get(filter).await?;

        Ok(Json(users))
    }
    pub async fn get_user_by_id(Path(id): Path<i32>,user_service: State<Arc<dyn UserService>>, Extension(identity): Extension<UserIdentity>)->Result<Json<User>,ApiError>{
        let user=user_service.get_by_id(id).await?;

        Ok(Json(user))
    }
    pub async fn delete_user_by_id(Path(id): Path<i32>,user_service: State<Arc<dyn UserService>>, Extension(identity): Extension<UserIdentity>)->Result<Json<i32>,ApiError>{
        let user_id=user_service.delete_by_id(id).await?;

        Ok(Json(user_id))
    }

    pub async fn create_user(Json(use_req): Json<CreateUserRequest>,user_service: State<Arc<dyn UserService>>, Extension(identity): Extension<UserIdentity>)->Result<Json<User>,ApiError>{
        let user=user_service.create(use_req,identity.user_id).await?;

        Ok(Json(user))
    }

}


pub fn user_router(state: Arc<AppState>)->Router{
    Router::new()
             .route("/{id}", get(UserHandler::get_user_by_id))
             .route("/{id}", delete(UserHandler::delete_user_by_id))
             .route("/",get(UserHandler::get_users))
             .with_state(state.user_container.user_service.clone())
 }