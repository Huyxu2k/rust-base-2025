use crate::{
    config::Config,
    diesel_impl::{self, user::UserDieselImpl},
    modules::{
        auth::{
            security::{SecurityService, SecurityServiceImpl},
            service::{AuthService, AuthServiceImpl},
        },
        user::{
            repository::UserRepo,
            service::{UserService, UserServiceImpl},
        },
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserContainer {
    pub user_service: Arc<dyn UserService>,
    pub user_service_impl: Arc<UserServiceImpl>,
    pub auth_service: Arc<dyn AuthService>,
    pub security_service: Arc<dyn SecurityService>,
    pub security_service_impl: Arc<SecurityServiceImpl>,
}

#[derive(Clone)]
pub struct PermissionContrainer {}

impl UserContainer {
    pub fn new(config: Config) -> Self {
        let pool = Arc::new(diesel_impl::db_pool::db_pool(config.clone().db.mysql));

        let user_repo: Arc<dyn UserRepo> = Arc::new(UserDieselImpl::new(pool));

        //service
        let security_service = Arc::new(SecurityServiceImpl {
            access_key: config.secret.access_secret ,
            refresh_key: config.secret.refresh_secret ,
        });

        UserContainer {
            user_service: todo!(),
            user_service_impl: todo!(),
            auth_service: todo!(),
            security_service: todo!(),
            security_service_impl: todo!(),
        }
    }
}
