

use serde::{Deserialize, Serialize};


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
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(sub: String)->Self{
        let now= chrono::Utc::now().timestamp();
        let exp= now+ EXPIRES;
        Self { sub, exp, iat: now }
    }
}


pub fn encode()