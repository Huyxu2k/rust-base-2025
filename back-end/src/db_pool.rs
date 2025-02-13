use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool,PooledConnection, PoolError};

use crate::config::MysqlConfig;


pub type DbPool= Pool<ConnectionManager<MysqlConnection>>;

//TODO use PoolError
pub fn establish_connection(config:MysqlConfig)->DbPool{
    let database_url=format!("mysql://{}:{}@{}:{}/{}",config.user,config.password,config.host,config.port,config.database);
    let manager= ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .max_size(config.max_connections)
        .build(manager)
        .expect("Failed to create pool")
}

// get conn
pub fn get_conn(pool: &DbPool) -> PooledConnection<ConnectionManager<MysqlConnection>> {
    pool.get().expect("Failed to get DB connection")
}

