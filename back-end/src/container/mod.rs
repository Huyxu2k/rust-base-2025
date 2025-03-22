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
    //user
    pub user_service: Arc<dyn UserService>,
    //auth
    pub auth_service: Arc<dyn AuthService>,
    //security
    pub security_service: Arc<dyn SecurityService>,
}

#[derive(Clone)]
pub struct PermissionContrainer {}

impl UserContainer {
    pub fn new(config: Config) -> Self {
        let pool = Arc::new(diesel_impl::db_pool::db_pool(config.clone().db.mysql));

        let user_repo: Arc<dyn UserRepo> = Arc::new(UserDieselImpl::new(pool));

        let security_service_impl = Arc::new(SecurityServiceImpl {
            access_key: config.secret.access_secret,
            refresh_key: config.secret.refresh_secret,
        });

        let user_service_impl = Arc::new(UserServiceImpl {
            user_repo: user_repo.clone(),
            security_service: security_service_impl.clone(),
        });

        let auth_service_impl= Arc::new(AuthServiceImpl{
            user_repo: user_repo.clone(),
            security_service: security_service_impl.clone(),
        });

        UserContainer {
            user_service: user_service_impl,
            auth_service: auth_service_impl,
            security_service: security_service_impl,
        }
    }
}
