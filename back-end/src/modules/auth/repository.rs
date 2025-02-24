use crate::{db_pool::get_conn, AppState};

use super::Token;
use crate::modules::user::repository::get_user_by_id;
use super::gen_token;


pub async fn create_token(user_id:i32,state: &AppState)->Result<Token,String>{
    let user= get_user_by_id(user_id,&state.pool).await.unwrap();
    let token=gen_token(&user,&state).await.unwrap();
    Ok(token)
}

pub async fn revoked_token(token_str: String){
 todo!()
}

pub async fn save_token(access_token: String, refresh_token: String){
    todo!()
}