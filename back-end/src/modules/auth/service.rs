use std::sync::Arc;

use async_trait::async_trait;

use crate::modules::user::repository::{User, UserRepo};

use super::security::SecurityService;




#[async_trait]
pub trait AuthService:Sync + Send {
    async fn login(&self, email_or_username: &str, password: &str) -> Result<(User, String), String>;
    async fn logout(&self,user_id:i32);
}


pub struct AuthServiceImpl{
  pub user_repo: Arc<dyn UserRepo>,
  pub security_service: Arc<dyn SecurityService>
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, email_or_username: &str, password: &str) -> Result<(User, String), String>{
        todo!()
    }
    async fn logout(&self,user_id:i32){
        todo!()
    }
}