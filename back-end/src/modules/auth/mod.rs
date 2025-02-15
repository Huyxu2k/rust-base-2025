

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::db_pool::DbPool;


pub const REFRESH_EXPIRES: i64=24*60*60;
pub const EXPIRES: i64= 4*60*60;


pub struct Token{
    user: String,
    access_token: String,
    refresh_token: String,
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Claims{
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(sub: String)->Self{
        let now= chrono::Utc::now().timestamp();
        let exp= now+ EXPIRES;
        Self { sub, exp, iat: now }
    }
}

pub fn encode_token(key:&str, claim:Claims)->Result<String,String>{
    let token= encode(&Header::default(), &claim, &EncodingKey::from_secret(&key.as_ref()));
    
    match token {
        Ok(tk) => Ok(tk),
        Err(e) => {
            Err(format!("Error encode token: {}",e))
        }
    }
}

pub fn decode_token(token:String,key:&str)->Result<TokenData<Claims>,String>{
   let result= decode::<Claims>(
             &token,
             &DecodingKey::from_secret(key.as_ref()),
             &Validation::default(),//new(Algorithm::HS256)
            );
    match result {
        Ok(de) => Ok(de),
        Err(e) => Err(format!("Error decode token: {}",e)),
    }
}
pub async fn verify_token(token_data:&TokenData<Claims>,pool: &DbPool)->Result<String,String>{
    todo!()
}