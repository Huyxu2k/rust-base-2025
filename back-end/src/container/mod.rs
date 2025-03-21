use crate::{config::Config, diesel_impl::{self, user::UserDieselImpl}, modules::{auth::{security::SecurityService, service::{AuthService, AuthServiceImpl}},user::{repository::UserRepo, service::{UserService, UserServiceImpl}}}};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserContainer{
  pub user_service_impl: Arc<UserServiceImpl>,
  pub auth_service_impl: Arc<AuthServiceImpl>,
  pub user_service: Arc<dyn UserService>,
  pub auth_service: Arc<dyn AuthService>,
  pub security_service: Arc<dyn SecurityService>
}

#[derive(Clone)]
pub struct PermissionContrainer{

}

impl UserContainer {
    pub fn new(config: Config)->Self{
      let pool= Arc::new(diesel_impl::db_pool::db_pool(config.db.mysql));

      let user_repo: Arc<dyn UserRepo> = Arc::new(UserDieselImpl::new(pool));
      UserContainer {
        user_service_impl: todo!(),
        auth_service_impl: todo!(),
        user_service: todo!(),
        auth_service: todo!(),
        security_service: todo!(),
    }
    }
}