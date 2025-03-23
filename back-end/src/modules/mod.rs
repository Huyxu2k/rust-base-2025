pub mod auth;
pub mod user;
pub mod permission;


use serde::{Deserialize, Serialize};
#[derive(Debug,Deserialize,Default)]
pub struct PaginationRequest{
    #[serde(default="default_limit")]
    pub limit: usize,
    #[serde(default="default_offset")]
    pub offset: usize
}

fn default_limit() -> usize {
    50
}

fn default_offset() -> usize {
    0
}

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct RepoError {
    pub message: String,
}

impl Into<CommonError> for RepoError {
    fn into(self) -> CommonError {
        CommonError {
            message: self.message,
            code: 1,
        }
    }
}
