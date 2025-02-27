use async_trait::async_trait;
use axum::{extract::Query, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait QueryParamsApi<T>
where
    T: Send + Sync + 'static,
{
    //async fn search(query: Query<T>) -> impl IntoResponse;
    async fn delete_by_ids(query: Query<T>) -> impl IntoResponse;
    async fn get(query: Query<T>) -> impl IntoResponse;
}

#[derive(Debug,Deserialize)]
pub struct Pagination{
    pub page: usize,
    pub per_page: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchParams {
    pub limit: i32,
    pub offset: i32,
    pub sort_order: String,
    pub sort_field: Option<String>,
}
impl Default for SearchParams {
    fn default() -> Self {
        Self {
            limit: 50,
            offset: 0,
            sort_order: "ASC".to_string(),
            sort_field: None,
        }
    }
}


