use std::sync::Arc;

use super::schema::_users;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use tokio::task;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name=_users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserDiesel {
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

impl Into<User> for UserDiesel {
    fn into(self) -> User {
        User {
            id: self.ID,
            employee_id: self.EmployeeId.unwrap_or(0),
            user_name: self.Username,
            password_hash: self.PasswordHash,
            email: self.Email,
            email_verified: self.EmailVerified.unwrap_or(false),
            is_active: self.IsActive.unwrap_or(true),
            created_by: self.CreatedBy,
            updated_by: self.UpdatedBy,
            created_at: self.CreatedAt.unwrap_or(NaiveDateTime::default()),
            updated_at: self.UpdatedAt.unwrap_or(NaiveDateTime::default()),
        }
    }
}
impl From<User> for UserDiesel {
    fn from(value: User) -> Self {
        UserDiesel {
            ID: value.id,
            EmployeeId: Some(value.employee_id),
            Username: value.user_name,
            PasswordHash: value.password_hash,
            Email: value.email,
            EmailVerified: Some(value.email_verified),
            IsActive: Some(value.is_active),
            CreatedBy: value.created_by,
            UpdatedBy: value.updated_by,
            CreatedAt: Some(value.created_at),
            UpdatedAt: Some(value.updated_at),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name=_users)]
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

use super::db_pool::DbConn;
use super::schema::_users::dsl::*;
use crate::modules::user::repository::UserRepo;
use crate::modules::user::repository::{
    CreateUserRequest, FilterUsersRequest, UpdateUserRequest, User,
};

pub struct UserDieselImpl {
    pool: Arc<DbConn>,
}
impl UserDieselImpl {
    pub fn new(pool: Arc<DbConn>) -> Self {
        UserDieselImpl { pool }
    }
    // fn get_conn(
    //     &self,
    // ) -> Result<PooledConnection<ConnectionManager<diesel::mysql::MysqlConnection>>, String> {
    //     let conn = self
    //         .pool
    //         .get()
    //         .map_err(|e| format!("Database connection error: {}", e))?;
    //     Ok(conn)
    // }
}

#[async_trait]
impl UserRepo for UserDieselImpl {
    async fn get(&self, filter: FilterUsersRequest) -> Result<Vec<User>, String> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Database connection error: {}", e))?;

            let result = _users::table
                .limit(filter.pagination.limit as i64)
                .offset(filter.pagination.offset as i64)
                .load::<UserDiesel>(&mut conn)
                .map_err(|e| e.to_string());

            result.map(|users| users.into_iter().map(|v| v.into()).collect())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }
    async fn get_by_id(&self, user_id: i32) -> Result<User, String> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Database connection error: {}", e))?;

            let result = _users
                .find(user_id)
                .first::<UserDiesel>(&mut conn)
                .map_err(|e| e.to_string());

            result.map(|user| user.into())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }
    async fn get_by_email_or_username(&self, email_or_username: String) -> Result<User, String> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Database connection error: {}", e))?;

            let result = _users::table
                .filter(_users::Username.eq(email_or_username.clone()))
                .or_filter(_users::Email.eq(email_or_username))
                .first::<UserDiesel>(&mut conn)
                .map_err(|e| e.to_string());

            result.map(|user| user.into())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }
    async fn create(&self, user: CreateUserRequest, user_id: i32) -> Result<User, String> {
        let pool = self.pool.clone();
        let inserted_id = task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Database connection error: {}", e))?;

            let new_user = NewUser {
                EmployeeId: None,
                Username: user.user_name,
                PasswordHash: user.password,
                Email: user.email,
                EmailVerified: Some(true),
                IsActive: Some(true),
                CreatedBy: user_id,
                UpdatedBy: user_id,
                CreatedAt: Some(chrono::Utc::now().naive_utc()),
                UpdatedAt: Some(chrono::Utc::now().naive_utc()),
            };

            let result = diesel::insert_into(_users)
                .values(&new_user)
                .execute(&mut conn)
                .map_err(|e| e.to_string())?;
            if result == 0 {
                return Err("Can't inserted".to_string());
            }
            let id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "LAST_INSERT_ID()",
            ))
            .get_result::<i32>(&mut conn)
            .map_err(|e| e.to_string());

            id
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))??;

        self.get_by_id(inserted_id).await.map_err(|e| e.to_string())
    }
    async fn update(&self, user_id:i32, user: UpdateUserRequest, by_id: i32) -> Result<User, String> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Database connection error: {}", e))?;

            let result = diesel::update(_users.find(user_id))
                .set(Username.eq(user.user_name))
                .execute(&mut conn)
                .map_err(|e| e.to_string())?;

            if result == 0 {
                return Err("Can't updated".to_string());
            }
            let user_update = _users
                .find(user_id)
                .first::<UserDiesel>(&mut conn)
                .map_err(|e| e.to_string());

            user_update.map(|user| user.into())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }
    async fn delete_by_id(&self, user_id: i32) -> Result<i32, String> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Database connection error: {}", e))?;

            let result = diesel::update(_users.find(user_id.clone()))
                .set(IsActive.eq(false))
                .execute(&mut conn)
                .map_err(|e|e.to_string())?;

            if result==0{
                return Err("Can't Delete".to_string());
            }
            Ok(user_id)
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }
    async fn delete_list_ids(&self, user_ids: Vec<i32>) -> Result<Vec<i32>, String> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Database connection error: {}", e))?;

            for id in user_ids.clone(){
                 diesel::update(_users.find(id))
                .set(IsActive.eq(false))
                .execute(&mut conn)
                .map_err(|e|e.to_string())?;
            }

            Ok(user_ids)
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }
}