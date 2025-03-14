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


// use async_trait::async_trait;
// use serde::Deserialize;
// use support_macro::openapi;
// use axum::{extract::{Path, State}, response::IntoResponse, Json};
// use axum_extra::extract::Query;
// use crate::{modules::traits::PaginationRequest, utils::extractor_body::JsonExtractor, AppState};
// use crate::utils::extractor_path::PathExtrator;

// use super::model::{CreateUserRequest, DeleteUsersRequest, FilterUsersRequest, UpdateUserRequest, User};
// use super::repository;


// use crate::modules::traits::{params_body::BodyParamsApi,params_query::QueryParamsApi,params_path::PathParamsApi};

// //Implement api trait for UserHandler 
// pub struct UserHandler;

// #[async_trait]
// impl PathParamsApi<i32> for UserHandler{
//     async fn get_by_id(Path(user_id): Path<i32>, State(app_state): State<AppState>)-> impl IntoResponse{
//         format!("Get Success :{}",user_id)
//     }
//     async fn delete_by_id(Path(user_id): Path<i32>, State(app_state): State<AppState>)-> impl IntoResponse{
//         format!("Delete Success:{}",user_id)
//     }
// }

// #[async_trait]
// impl QueryParamsApi<DeleteUsersRequest,FilterUsersRequest> for UserHandler {
//     async fn delete_by_ids(Query(query): Query<DeleteUsersRequest>, State(app_state): State<AppState>) -> impl IntoResponse {
//         format!("Deletes Success: {:?}",query.ids)
//     }

//     async fn get(Query(query): Query<FilterUsersRequest>, State(app_state): State<AppState>) -> impl IntoResponse {
//         format!("Get users with filter: {:?}", query.name)
//     }
// }

// #[async_trait]
// impl BodyParamsApi<UpdateUserRequest,CreateUserRequest> for UserHandler {
//     async fn update(Json(body): Json<UpdateUserRequest>,app_state: State<AppState>)-> impl IntoResponse{
//         let data=repository::update_user(body, &app_state.pool).await.unwrap();
        
//         TResponse<User>
//     }
//     async fn create(body: Json<CreateUserRequest>,app_state: State<AppState>)-> impl IntoResponse{
//         todo!()
//     }
// }
