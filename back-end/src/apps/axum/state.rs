use chrono::{DateTime, NaiveDateTime, TimeZone};
use config::Config;

use crate::{config, container::UserContainer};


#[derive(Clone)]
pub struct AppState{
    pub user_container: UserContainer
}

impl AppState {
    pub async fn new(config: Config)->Self{
        AppState{
            user_container: UserContainer::new(config.clone()),
        }
    }
}

//date convert
pub fn convert_to_datetime_utc_x<T: TimeZone>(naive:NaiveDateTime,time_zone:T)->DateTime<T>{
    time_zone.from_utc_datetime(&naive)
}
