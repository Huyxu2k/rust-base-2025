use async_trait::async_trait;
use axum::{extract::{Query, State}, response::IntoResponse};
use crate::AppState;

#[async_trait]
pub trait QueryParamsApi
{
    //async fn search(query: Query<T>) -> impl IntoResponse;
    async fn delete_by_ids<T>(query: Query<T>,app_state: State<AppState>) -> impl IntoResponse
    where 
      T: Send+ Sync+ 'static;

    async fn get<S>(query: Query<S>,app_state: State<AppState>) -> impl IntoResponse
    where 
      S: Send+ Sync+ 'static;
}




