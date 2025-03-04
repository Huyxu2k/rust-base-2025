use axum::routing::{delete, get};
use axum::Router;

use crate::modules::traits::params_path::PathParamsApi;
use crate::modules::user::handle::UserHandler;
use crate::AppState;
pub fn user_router(state: AppState)->Router{
   Router::new()
            .route("/{id}", get(UserHandler::get_by_id))
            .route("/{id}", delete(UserHandler::delete_by_id))
            .with_state(state)
}