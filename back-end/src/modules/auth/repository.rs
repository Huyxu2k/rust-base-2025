use crate::db_pool::DbPool;

use super::Token;


pub async fn create_token(pool: &DbPool)->Result<Token,String>{
    
todo!()
}

pub async fn revoked_token(token_str: String){
 todo!()
}

pub async fn save_token(access_token: String, refresh_token: String){
    todo!()
}