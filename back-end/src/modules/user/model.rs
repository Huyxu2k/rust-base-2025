
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::modules::traits::PaginationRequest;


#[derive(Queryable,Selectable,Debug)]
#[diesel(table_name= crate::schema::_users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User{
    pub ID: i32,
    pub EmployeeId: Option<i32>,
    pub Username: String,
    pub PasswordHash: String,
    pub Email: String,
    pub EmailVerified: Option<bool>,
    pub IsActive: Option<bool>,
    pub CreatedBy: i32,
    pub UpdatedBy: i32,
    pub CreatedAt: Option<NaiveDateTime>,
    pub UpdatedAt: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::_users)]
pub struct NewUser {
    pub EmployeeId: Option<i32>,
    pub Username: String,
    pub PasswordHash: String,
    pub Email: String,
    pub EmailVerified: Option<bool>,
    pub IsActive: Option<bool>,
    pub CreatedBy: i32,
    pub UpdatedBy: i32,
    pub CreatedAt: Option<chrono::NaiveDateTime>,
    pub UpdatedAt: Option<chrono::NaiveDateTime>,
}


#[derive(Debug,Deserialize,Serialize)]
pub struct UserRegister{
    pub user_name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct LoginRequest{
    pub account: String,
    pub password: String,
}

pub struct ModifyPasswordRequest{
    pub user_id: i32,
    pub password: String
}

//User Request

pub struct UserFilter{
    pub name: Option<String>,
    pub pagination: Option<PaginationRequest>
}