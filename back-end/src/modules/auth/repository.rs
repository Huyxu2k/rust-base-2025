use crate::{db_pool::{get_conn, DbPool}, AppState};

use super::Token;
use crate::modules::user::repository::get_user_by_username_or_email;
use super::gen_token;


pub async fn create_token(name: String, password:String,state: &AppState)->Result<Token,String>{
    let user= get_user_by_username_or_email(name,&state.pool).await.unwrap();
    let token=gen_token(&user,&state).await.unwrap();
    Ok(token)
}

pub async fn revoked_token(token_str: String){
 todo!()
}

pub async fn save_token(access_token: String, refresh_token: String){
    todo!()
}