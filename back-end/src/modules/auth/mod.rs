pub mod model;

use axum::Json;
use chrono::DateTime;
use diesel::{query_dsl::methods::{FilterDsl, FindDsl, SelectDsl}, ExpressionMethods, IntoSql, OptionalExtension, RunQueryDsl, SelectableHelper};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::{db_pool::{get_conn, DbPool},schema::{_refresh_tokens::dsl::*, _users::dsl::*},AppState};
use super::user::model::User;
use super::auth::model::RefreshToken;
use crate::schema::_refresh_tokens;
use crate::schema::_users;


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
pub async fn verify_access_token(token: String,pool: &DbPool)->Result<String,String>{
    //check access token
    todo!()
}

pub async fn verify_refresh_token(refresh_token:String, state: &AppState )->Result<Token,String>{
    //get UUID by refresh token in db
    let mut conn=get_conn(&state.pool);
    let orefresh_token=_refresh_tokens::table
                                    .filter(_refresh_tokens::RefreshToken.eq(&refresh_token))
                                    .first::<RefreshToken>(&mut conn)
                                    .optional().unwrap();
                                
    match orefresh_token {
        Some(token) => {
            if token.Expiry<chrono::Utc::now().naive_utc(){
               let user= _users.find(token.UserID)
                                            .select(User::as_select())
                                            .first::<User>(&mut conn)
                                            .optional().unwrap();
                match user {
                    Some(user) => {
                        let token=gen_token(&user, state).await.unwrap();
                        Ok(token)
                    },
                    None => Err(format!("Not found user by id:{}",token.UserID)),
                }
            }else {
                Err(format!("Refresh token is expired!"))
            }
        },
        None => {
            Err(format!("Not found refresh token: {}",refresh_token))
        },
    }
}

pub async fn gen_token(user: &User, state: &AppState)->Result<Token,String>{
    let mut claim= Claims::new(user.ID.to_string());
    let access_token= encode_token(&state.jwt_secret, claim.clone()).unwrap();

    //TODO
    let id=uuid::Uuid::new_v4();
    claim.exp +=REFRESH_EXPIRES;
    let refresh_token= encode_token(&state.jwt_secret, claim).unwrap();

    //TODO Save to Db
    Ok(Token { user: user.Username.to_string(), 
                access_token: access_token,
                refresh_token: refresh_token 
    })
}