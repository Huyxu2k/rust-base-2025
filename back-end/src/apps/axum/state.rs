use std::sync::Arc;

use chrono::{DateTime, NaiveDateTime, TimeZone};
use config::Config;

use crate::{config, container::UserContainer};


#[derive(Clone)]
pub struct AppState{
    pub user_container: UserContainer
}

impl AppState {
    pub fn new(config: Config)->AppState{
        AppState{
            user_container: UserContainer::new(config.clone()),
        }
    }
}
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}
//date convert
pub fn convert_to_datetime_utc_x<T: TimeZone>(naive:NaiveDateTime,time_zone:T)->DateTime<T>{
    time_zone.from_utc_datetime(&naive)
}
