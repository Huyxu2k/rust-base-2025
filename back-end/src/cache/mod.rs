use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use axum::http::Error;

use crate::config::Config;

mod redis;


#[async_trait]
pub trait Cache: Sync + Send + Debug {

    async fn check_role_permission(&self, permission: i64)-> Result<bool, Error>;

    async fn get_role_permission(&self)->Result<Vec<String>, Error>;

    async fn save_role_permission(&self, role_id: i64,permissions: Vec<i64>)-> Result<(), Error>;

    async fn remove_role_permission(&self, role_id: i64)-> Result<(), Error>;
}

pub fn cache(config: & Config)-> Arc<dyn Cache>{
    Arc::new(redis::RedisCache::from_config(config))
}