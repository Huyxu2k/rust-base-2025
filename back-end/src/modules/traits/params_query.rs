use async_trait::async_trait;
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::Query;
use serde::de::DeserializeOwned;
use crate::AppState;

#[async_trait]
pub trait QueryParamsApi<T,S>
where 
    T: Send + Sync + 'static + DeserializeOwned,
    S: Send+ Sync+ 'static + DeserializeOwned
{
    //async fn search(query: Query<T>) -> impl IntoResponse;
    async fn delete_by_ids(query: Query<T>,app_state: State<AppState>) -> impl IntoResponse;
    async fn get(query: Query<S>,app_state: State<AppState>) -> impl IntoResponse;
}




