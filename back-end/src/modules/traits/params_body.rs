use async_trait::async_trait;
use axum::{extract::{Json, State}, response::IntoResponse};

use crate::AppState;

#[async_trait]
pub trait BodyParamsApi
{
    async fn update<T>(body: Json<T>,app_state: State<AppState>)-> impl IntoResponse
    where 
      T: Send+ Sync+ 'static;
    async fn create<S>(body: Json<S>,app_state: State<AppState>)-> impl IntoResponse
    where 
      S: Send+ Sync+ 'static;
}


