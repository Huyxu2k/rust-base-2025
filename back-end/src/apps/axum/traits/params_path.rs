use async_trait::async_trait;
use axum::{extract::{Path, State}, response::IntoResponse};

use crate::{utils::extractor_path::PathExtrator, AppState};

#[async_trait]
pub trait PathParamsApi<T> 
where 
    T: Send+ Sync+ 'static
{
    async fn get_by_id(path: Path<T>, app_state: State<AppState>)-> impl IntoResponse;
    async fn delete_by_id(path: Path<T>, app_state: State<AppState>)-> impl IntoResponse;
}