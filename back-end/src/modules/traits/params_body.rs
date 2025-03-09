use async_trait::async_trait;
use axum::{extract::{Json, State}, response::IntoResponse};
use serde::de::DeserializeOwned;

use crate::AppState;

#[async_trait]
pub trait BodyParamsApi<T,S>
where 
      T: Send+ Sync+ 'static + DeserializeOwned,
      S: Send+ Sync+ 'static + DeserializeOwned
{
    async fn update(body: Json<T>,app_state: State<AppState>)-> impl IntoResponse;
    async fn create(body: Json<S>,app_state: State<AppState>)-> impl IntoResponse;
}


