use async_trait::async_trait;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::modules::{user::repository::User, CommonError, RepoError};

pub const REFRESH_EXPIRES: i64 = 24 * 60 * 60;
pub const EXPIRES: i64 = 4 * 60 * 60;

pub struct Token {
    pub user: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        let exp = now + EXPIRES;
        Self { sub, exp, iat: now }
    }
}
#[async_trait]
pub trait SecurityService: Send + Sync {
    async fn hash(&self, input: &str) -> Result<String, CommonError>;

    async fn verify_hash(&self, hashed: &str, raw: &str) -> Result<bool, CommonError>;

    async fn verify_token(&self, token_type: TypeToken, token: String)-> Result<bool, CommonError>;

    async fn token_generator(&self, user: &User) -> Result<Token, CommonError>;

    async fn decode_token(&self, token_type: TypeToken, token: &str) -> Result<TokenData<Claims>, CommonError>;

    async fn encode_token(&self, token_type: TypeToken, claim: Claims) -> Result<String, CommonError>;
}
pub struct SecurityServiceImpl {
    pub access_key: String,
    pub refresh_key: String,
}
pub enum TypeToken {
    Access,
    Refresh,
}

#[async_trait]
impl SecurityService for SecurityServiceImpl {
    async fn hash(&self, value: &str) -> Result<String, CommonError> {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    async fn verify_hash(&self, hashed: &str, pass: &str) -> Result<bool, CommonError> {
        let hashed_curr = self.hash(pass).await?;
        Ok(hashed_curr == hashed)
    }

    async fn verify_token(&self, token_type: TypeToken, token: String)-> Result<bool, CommonError>{
        match token_type {
            TypeToken::Access => {
                let decode_claim = self.decode_token(TypeToken::Access,&token).await.map_err(|e| e.into())?;
                if decode_claim.claims.exp >= chrono::Utc::now().timestamp() {
                    Ok(true)
                } else {
                    Err(RepoError{message: "Access token is expired!".to_string()}.into())
                }
            }
            TypeToken::Refresh => {
                let decode_claim =self.decode_token(TypeToken::Refresh,&token).await.map_err(|e| e.into())?;
                if decode_claim.claims.exp >= chrono::Utc::now().timestamp() {
                    Ok(true)
                } else {
                    Err(RepoError{message: "Refresh token is expired!".to_string()}.into())
                }
            }
        }
    }

    async fn token_generator(&self, user: &User) -> Result<Token, CommonError> {
        let time_origin = chrono::Utc::now();
        let now = time_origin.clone().timestamp();
        let exp = now + EXPIRES;
        let mut claim = Claims {
            sub: user.id.to_string(),
            exp,
            iat: now,
        };
        let access_token_string = self
            .encode_token(TypeToken::Access, claim.clone())
            .await
            .map_err(|e| e.into())?;

        claim.exp = now + REFRESH_EXPIRES;
        let refresh_token_string = self
            .encode_token(TypeToken::Refresh, claim)
            .await
            .map_err(|e| e.into())?;

        Ok(Token {
            user: user.user_name.to_string(),
            access_token: access_token_string,
            refresh_token: refresh_token_string,
        })
    }

    async fn decode_token(&self, token_type: TypeToken, token: &str) -> Result<TokenData<Claims>, CommonError> {
        let result;
        match token_type {
            TypeToken::Access => {
                result = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(&self.access_key.as_ref()),
                    &Validation::default(), //new(Algorithm::HS256)
                );
            }
            TypeToken::Refresh => {
                result = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(&self.refresh_key.as_ref()),
                    &Validation::default(), //new(Algorithm::HS256)
                );
            }
        }
        match result {
            Ok(decode) => Ok(decode),
            Err(e) =>Err(RepoError{message: format!("Error decode token: {}", e)}.into()),
        }
    }

    async fn encode_token(&self, token_type: TypeToken, claim: Claims) -> Result<String, CommonError> {
        let token;
        match token_type {
            TypeToken::Access => {
                token = encode(
                    &Header::default(),
                    &claim,
                    &EncodingKey::from_secret(&self.access_key.as_ref()),
                );
            }
            TypeToken::Refresh => {
                token = encode(
                    &Header::default(),
                    &claim,
                    &EncodingKey::from_secret(&self.refresh_key.as_ref()),
                );
            }
        }
        match token {
            Ok(token) => Ok(token),
            Err(e) => Err(RepoError{message: format!("Error encode token: {}", e)}.into()),
        }
    }
}
