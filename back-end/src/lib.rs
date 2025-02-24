
pub mod error;
pub mod middleware;
pub mod utils;
pub mod cache;
pub mod db_pool;
pub mod config;
pub mod routes;
pub mod modules;

pub mod schema;

use chrono::{DateTime, NaiveDateTime, TimeZone};
use config::Config;
use db_pool::{establish_connection, DbPool};
use sha2::{Sha256,Digest};
use std::{net::SocketAddr, sync::Arc};


#[derive(Clone)]
pub struct AppState{
    pub pool: Arc<DbPool>,
    pub jwt_secret: String,
    pub secret: String
}

impl AppState {
    pub async fn new(config: Config)->AppState{
        let pool=Arc::new(establish_connection(config.db.mysql));

        AppState { 
            pool, 
            jwt_secret: "test".to_string(),
            secret:"hello".to_string()
        }
    }
}

pub async fn start(config: Config){
    let state= AppState::new(config.clone()).await;
    let app= routes::app_routes(state);

    let listener= tokio::net::TcpListener::bind(&config.server.server_url())
                .await
                .unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()   
    )
    .await
    .unwrap();

}

//date convert
pub fn convert_to_datetime_utc_x<T: TimeZone>(naive:NaiveDateTime,time_zone:T)->DateTime<T>{
    time_zone.from_utc_datetime(&naive)
}

//hash
pub fn to_hash(value: String)->String{
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}