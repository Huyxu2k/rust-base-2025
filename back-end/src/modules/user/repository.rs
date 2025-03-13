
use async_trait::async_trait;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::modules::traits::PaginationRequest;

// Repository model
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct User{
    pub id: i32,
    pub employee_id: i32,
    pub user_name: String,
    pub password_hash: String,
    pub email: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_by: i32,
    pub updated_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

pub struct UserIdentity{
    pub user_id: i32,
    pub email: String,
    pub role: String
}

//Request model 

/// Delete users by id
#[derive(Deserialize,Debug)]
pub struct DeleteUsersRequest{
    #[serde(default)]
    pub ids: Vec<i32>
}

/// Filter users
#[derive(Deserialize,Debug)]
pub struct FilterUsersRequest{
    pub name: Option<String>,
    pub pagination:PaginationRequest,
}

/// Create user
#[derive(Deserialize,Debug)]
pub struct CreateUserRequest{
    pub user_name: String,
    pub email: String,
    pub password: String
}

/// Update user
#[derive(Deserialize,Debug)]
pub struct UpdateUserRequest{
    pub id: i32,
    pub employee_id: i32,
    pub user_name: String,
    pub password: String,
    pub email: String,
}

#[async_trait]
pub trait UserRepo: Send+ Sync{
  async fn get(&self,filter:FilterUsersRequest)->Result<Vec<User>,String>;
  async fn get_by_id(&self, user_id:i32)->Result<User,String>;
  async fn get_by_email_or_username(&self,email_or_username:String)->Result<User, String>;
  async fn create(&self,user:CreateUserRequest,user_id: i32)->Result<User,String>;
  async fn update(&self,user:UpdateUserRequest,user_id: i32)->Result<User,String>;
  async fn delete_by_id(&self,user_id:i32)->Result<i32,String>;
  async fn delete_list_ids(&self, user_ids: Vec<i32>)->Result<Vec<i32>,String>;
}