use std::sync::Arc;

use async_trait::async_trait;

use crate::modules::auth::{repository::AuthRepo, service::AuthService};

use super::repository::{CreateUserRequest, FilterUsersRequest, UpdateUserRequest, User, UserRepo};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get(&self, filter: FilterUsersRequest) -> Result<Vec<User>, String>;
    async fn get_by_id(&self, user_id: i32) -> Result<User, String>;
    async fn get_by_email_or_username(&self, email_or_username: String) -> Result<User, String>;
    async fn create(&self, user: CreateUserRequest, by_id: i32) -> Result<User, String>;
    async fn update(
        &self,
        user_id: i32,
        user: UpdateUserRequest,
        by_id: i32,
    ) -> Result<User, String>;
    async fn delete_by_id(&self, user_id: i32) -> Result<i32, String>;
    async fn delete_list_ids(&self, user_ids: Vec<i32>) -> Result<Vec<i32>, String>;
    //add logic here
}

pub struct UserServiceImpl {
    pub user_repo: Arc<dyn UserRepo>,
    pub auth_service: Arc<dyn AuthService>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get(&self, filter: FilterUsersRequest) -> Result<Vec<User>, String> {
        self.user_repo.get(filter).await.map_err(|e| e.to_string())
    }
    async fn get_by_id(&self, user_id: i32) -> Result<User, String> {
        self.user_repo.get_by_id(user_id).await.map_err(|e|e.to_string())
    }
    async fn get_by_email_or_username(&self, email_or_username: String) -> Result<User, String> {
        todo!();
    }
    async fn create(&self, user: CreateUserRequest, by_id: i32) -> Result<User, String> {
        self.user_repo.create(user, by_id).await.map_err(|e|e.to_string())
    }
    async fn update(
        &self,
        user_id: i32,
        user: UpdateUserRequest,
        by_id: i32,
    ) -> Result<User, String>{
        self.user_repo.update(user_id, user, by_id).await.map_err(|e|e.to_string())
    }
    async fn delete_by_id(&self, user_id: i32) -> Result<i32, String> {
        todo!();
    }
    async fn delete_list_ids(&self, user_ids: Vec<i32>) -> Result<Vec<i32>, String> {
        todo!();
    }
}
