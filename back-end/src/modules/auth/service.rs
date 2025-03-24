use std::sync::Arc;

use async_trait::async_trait;

use crate::modules::{user::repository::{User, UserRepo}, CommonError};

use super::security::SecurityService;




#[async_trait]
pub trait AuthService:Sync + Send {
    async fn login(&self, email_or_username: &str, password: &str) -> Result<(User, String), CommonError>;
    async fn logout(&self,user_id:i32);
}


pub struct AuthServiceImpl{
  pub user_repo: Arc<dyn UserRepo>,
  pub security_service: Arc<dyn SecurityService>
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, email_or_username: &str, password: &str) -> Result<(User, String), CommonError>{
        let user= self.user_repo.get_by_email_or_username(email_or_username.to_string()).await.map_err(|e|e.into())?;

        let token=self.security_service.token_generator(&user).await.map_err(|e|e.into())?;

        Ok((user,token.access_token))
    }
    async fn logout(&self,user_id:i32){
        todo!()
    }
}