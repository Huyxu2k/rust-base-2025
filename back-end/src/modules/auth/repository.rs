use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait AuthRepo: Send+ Sync{
  async fn revoked_token(&self,token_str: String)->Result<bool,String>;
  async fn save_token(&self, access_token:String,refresh_token: String)->Result<bool,String>;
}

//request
#[derive(Debug,Deserialize)]
pub struct UserLogin{
    pub UserNameOrEmail: String,
    pub Password: String
}


// pub async fn revoked_token(
//   token: String,
//   token_type: TypeToken,
//   pool: &DbPool,
// ) -> Result<i32, String> {
//   let mut conn = get_conn(&pool);
//   match token_type {
//       TypeToken::Access => {
//           let token = _access_tokens::table
//               .filter(_access_tokens::AccessToken.eq(&token))
//               .first::<AccessToken>(&mut conn)
//               .optional()
//               .unwrap();

//           match token {
//               Some(access_token) => {
//                   diesel::update(_access_tokens.find(access_token.ID))
//                       .set(_access_tokens::Revoked.eq(true))
//                       .execute(&mut conn);
//                   Ok(access_token.UserID)
//               }
//               None => Err(format!("Can't revoked access token")),
//           }
//       }
//       TypeToken::Refresh => {
//           let token = _refresh_tokens::table
//               .filter(_refresh_tokens::RefreshToken.eq(&token))
//               .first::<RefreshToken>(&mut conn)
//               .optional()
//               .unwrap();

//           match token {
//               Some(refresh_token) => {
//                   diesel::update(_refresh_tokens.find(refresh_token.ID))
//                       .set(_refresh_tokens::Revoked.eq(true))
//                       .execute(&mut conn);
//                   Ok(refresh_token.UserID)
//               }
//               None => Err(format!("Can't revoked refresh token")),
//           }
//       }
//   }
// }

// pub async fn check_revoked(
//   token: String,
//   token_type: TypeToken,
//   pool: &DbPool,
// ) -> Result<bool, String> {
//   let mut conn = get_conn(&pool);
//   match token_type {
//       TypeToken::Access => {
//           let token = _access_tokens::table
//               .filter(_access_tokens::AccessToken.eq(&token))
//               .first::<AccessToken>(&mut conn)
//               .optional()
//               .unwrap();

//           match token {
//               Some(access_token) => Ok(access_token.Revoked.unwrap_or(true)),
//               None => Err(format!("Can't find access token")),
//           }
//       }
//       TypeToken::Refresh => {
//           let token = _refresh_tokens::table
//               .filter(_refresh_tokens::RefreshToken.eq(&token))
//               .first::<RefreshToken>(&mut conn)
//               .optional()
//               .unwrap();

//           match token {
//               Some(refresh_token) => Ok(refresh_token.Revoked.unwrap_or(true)),
//               None => Err(format!("Can't find refresh token")),
//           }
//       }
//   }
// }