use chrono::{DateTime, NaiveDateTime, TimeZone};
use config::Config;
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

//date convert
pub fn convert_to_datetime_utc_x<T: TimeZone>(naive:NaiveDateTime,time_zone:T)->DateTime<T>{
    time_zone.from_utc_datetime(&naive)
}
