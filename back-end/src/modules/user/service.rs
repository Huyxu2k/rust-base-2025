use std::sync::Arc;

use async_trait::async_trait;

use crate::modules::{auth::security::SecurityService, CommonError};

use super::repository::{CreateUserRequest, FilterUsersRequest, UpdateUserRequest, User, UserRepo};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get(&self, filter: FilterUsersRequest) -> Result<Vec<User>, CommonError>;
    async fn get_by_id(&self, user_id: i32) -> Result<User, CommonError>;
    //async fn get_by_email_or_username(&self, email_or_username: String) -> Result<User, String>;
    async fn create(&self, user: CreateUserRequest, by_id: i32) -> Result<User, CommonError>;
    async fn update(
        &self,
        user_id: i32,
        user: UpdateUserRequest,
        by_id: i32,
    ) -> Result<User, CommonError>;
    async fn delete_by_id(&self, user_id: i32) -> Result<i32, CommonError>;
    async fn delete_list_ids(&self, user_ids: Vec<i32>) -> Result<Vec<i32>, CommonError>;
    //async fn update_password(&self, user_id: i32, )
    //add logic here
}

pub struct UserServiceImpl {
    pub user_repo: Arc<dyn UserRepo>,
    pub security_service: Arc<dyn SecurityService>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get(&self, filter: FilterUsersRequest) -> Result<Vec<User>, CommonError> {
        self.user_repo.get(filter).await.map_err(|e| e.into())
    }
    async fn get_by_id(&self, user_id: i32) -> Result<User, CommonError> {
        self.user_repo.get_by_id(user_id).await.map_err(|e|e.into())
    }
    // async fn get_by_email_or_username(&self, email_or_username: String) -> Result<User, String> {
    //     todo!();
    // }
    async fn create(&self, user: CreateUserRequest, by_id: i32) -> Result<User, CommonError> {
        self.user_repo.create(user, by_id).await.map_err(|e|e.into())
    }
    async fn update(
        &self,
        user_id: i32,
        user: UpdateUserRequest,
        by_id: i32,
    ) -> Result<User, CommonError>{
        self.user_repo.update(user_id, user, by_id).await.map_err(|e|e.into())
    }
    async fn delete_by_id(&self, user_id: i32) -> Result<i32, CommonError> {
        self.user_repo.delete_by_id(user_id).await.map_err(|e|e.into())
    }
    async fn delete_list_ids(&self, user_ids: Vec<i32>) -> Result<Vec<i32>, CommonError> {
        self.user_repo.delete_list_ids(user_ids).await.map_err(|e|e.into())
    }
}
