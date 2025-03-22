use axum::routing::{delete, get};
use axum::Router;
use crate::apps::axum::state::AppState;
use serde::Deserialize;
use support_macro::openapi;
use axum::{extract::{Path, State}, response::IntoResponse, Json};
use axum_extra::extract::Query;
use crate::utils::extractor_path::PathExtrator;

use crate::modules::user::repository::{CreateUserRequest, DeleteUsersRequest, FilterUsersRequest, UpdateUserRequest, User};

//Implement api trait for UserHandler 
pub struct UserHandler;

impl UserHandler {
    //todo
}


pub fn user_router(state: AppState)->Router{
    Router::new()
            //  .route("/{id}", get(UserHandler::get_by_id))
            //  .route("/{id}", delete(UserHandler::delete_by_id))
            //  .route("/",delete(UserHandler::delete_by_ids))
            //  .route("/",get(UserHandler::get))
             .with_state(state)
 }