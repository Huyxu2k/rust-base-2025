use crate::modules::{auth::service::{AuthService, AuthServiceImpl},user::service::{UserService, UserServiceImpl}};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserContainer{
  pub user_service_impl: Arc<UserServiceImpl>,
  pub auth_service_impl: Arc<AuthServiceImpl>,
  pub user_service: Arc<dyn UserService>,
  pub auth_service: Arc<AuthService>
}

#[derive(Clone)]
pub struct PermissionContrainer{

}