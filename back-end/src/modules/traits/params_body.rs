use async_trait::async_trait;
use axum::{extract::Json, response::IntoResponse};

#[async_trait]
pub trait BodyParamsApi<T> 
where 
    T: Send+ Sync+ 'static
{
    async fn update(body: Json<T>)-> impl IntoResponse;
    async fn create(body: Json<T>)-> impl IntoResponse;
}


