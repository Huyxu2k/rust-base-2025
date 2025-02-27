use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;


#[derive(Queryable,Selectable,Debug)]
#[diesel(table_name= crate::schema::_refresh_tokens)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RefreshToken{
    pub ID: i32,
    pub UserID: i32,
    pub RefreshToken: String,
    pub Expiry: NaiveDateTime,
    pub IPAddress: Option<String>,
    pub UserAgent: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub Revoked: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::_refresh_tokens)]
pub struct NewRefreshToken{
    pub UserID: i32,
    pub RefreshToken: String,
    pub Expiry: NaiveDateTime,
    pub IPAddress: Option<String>,
    pub UserAgent: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub Revoked: Option<bool>,
}


#[derive(Queryable,Selectable,Debug)]
#[diesel(table_name= crate::schema::_access_tokens)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct AccessToken{
    pub ID: i32,
    pub UserID: i32,
    pub AccessToken: String,
    pub Expiry: NaiveDateTime,
    pub IPAddress: Option<String>,
    pub UserAgent: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub Revoked: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::_access_tokens)]
pub struct NewAccessToken{
    pub UserID: i32,
    pub AccessToken: String,
    pub Expiry: NaiveDateTime,
    pub IPAddress: Option<String>,
    pub UserAgent: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub Revoked: Option<bool>,
}

//request

#[derive(Debug,Deserialize)]
pub struct UserLogin{
    pub UserNameOrEmail: String,
    pub Password: String
}