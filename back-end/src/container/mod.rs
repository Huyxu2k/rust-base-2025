use crate::modules::{auth,user::service::UserServiceImpl};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserContainer{
  pub user_service_impl: Arc<UserServiceImpl>,
}

#[derive(Clone)]
pub struct PermissionContrainer{

}