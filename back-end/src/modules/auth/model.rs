use serde::Deserialize;
//request

#[derive(Debug,Deserialize)]
pub struct UserLogin{
    pub UserNameOrEmail: String,
    pub Password: String
}