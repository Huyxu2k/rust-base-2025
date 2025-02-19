use crate::db_pool::DbPool;

use super::Token;


pub async fn create_token(pool: &DbPool)->Result<Token,String>{
    
// let orefresh_token=_refresh_tokens::table
// .filter(_refresh_tokens::RefreshToken.eq(&refresh_token))
// .first::<RefreshToken>(&mut conn)
// .optional().unwrap();
// match orefresh_token {
// Some(token) => {
//     let user= _users.find(token.UserID)
//                          .select(User::as_select())
//                          .first::<User>(&mut conn)
//                          .optional().unwrap();
//     match user {
//         Some(user) => {
//             let token=gen_token(&user, state).await.unwrap();
//             Ok(token)
//         },
//         None => Err(format!("Not found user by id:{}",token.UserID)),
//     }
// },
// None => {
//     Err(format!("Not found refresh token"))
// },
// }
todo!()
}

pub async fn revoked_token(token_str: String){
 todo!()
}

pub async fn save_token(access_token: String, refresh_token: String){
    todo!()
}