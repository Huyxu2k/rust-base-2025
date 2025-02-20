pub mod model;
pub mod repository
;
pub mod api;

use diesel::RunQueryDsl;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use model::{NewAccessToken, NewRefreshToken};
use serde::{Deserialize, Serialize};
use crate::{db_pool::get_conn,schema::{_access_tokens::dsl::*, _refresh_tokens::dsl::*},AppState};
use super::user::model::User;


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
pub async fn verify_access_token(token: String,state: &AppState)->Result<bool,String>{
    let decode_claim=decode_token(token,&state.jwt_secret).unwrap();
                                
    if decode_claim.claims.exp<=chrono::Utc::now().timestamp(){
        Ok(true)
     }else {
         Err(format!("Access token is expired!"))
     }
}

pub async fn verify_refresh_token(refresh_token:String, state: &AppState )->Result<bool,String>{
    let decode_claim=decode_token(refresh_token,&state.secret).unwrap();
                                
    if decode_claim.claims.exp<=chrono::Utc::now().timestamp(){
        Ok(true)
     }else {
         Err(format!("Refresh token is expired!"))
     }
}

pub async fn revoked_token(access_token: String,refresh_token: String ){
    todo!()
}

pub async fn gen_token(user: &User, state: &AppState)->Result<Token,String>{
    let mut conn=get_conn(&state.pool);

    let time_origin=chrono::Utc::now();
    let now= time_origin.clone().timestamp();
    let exp= now+ EXPIRES;
    let mut claim= Claims{
        sub: user.ID.to_string(),
        exp,
        iat: now,
    };
    let access_token_string= encode_token(&state.jwt_secret, claim.clone()).unwrap();

    claim.exp= now+ REFRESH_EXPIRES;
    let refresh_token_string=encode_token(&state.secret, claim).unwrap();

    let refresh_token=NewRefreshToken{
        UserID: user.ID.clone(),
        RefreshToken: refresh_token_string.clone(),
        Expiry: time_origin.clone().naive_utc(),
        IPAddress: None,
        UserAgent: None,
        CreatedAt: Some(time_origin.naive_utc()),
        Revoked: Some(false)
    };

    
    let access_token=NewAccessToken{
        UserID: user.ID.clone(),
        AccessToken: access_token_string.clone(),
        Expiry: time_origin.clone().naive_utc(),
        IPAddress: None,
        UserAgent: None,
        CreatedAt: Some(time_origin.naive_utc()),
        Revoked: Some(false)
    };
    
    //Save refresh token 
    diesel::insert_into(_refresh_tokens)
            .values(refresh_token)
            .execute(&mut conn).unwrap();
    //access token
    diesel::insert_into(_access_tokens)
            .values(access_token)
            .execute(&mut conn).unwrap();

    Ok(Token { user: user.Username.to_string(), 
                access_token: access_token_string,
                refresh_token: refresh_token_string 
    })
}