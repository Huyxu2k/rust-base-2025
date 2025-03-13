// use axum::routing::{delete, get};
// use axum::Router;

// use crate::modules::traits::params_path::PathParamsApi;
// use crate::modules::traits::params_query::QueryParamsApi;
// use crate::modules::user::handler::UserHandler;
// use crate::AppState;
// pub fn user_router(state: AppState)->Router{
//    Router::new()
//             .route("/{id}", get(UserHandler::get_by_id))
//             .route("/{id}", delete(UserHandler::delete_by_id))
//             .route("/",delete(UserHandler::delete_by_ids))
//             .route("/",get(UserHandler::get))
//             .with_state(state)
// }