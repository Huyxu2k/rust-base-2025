pub mod auth;
pub mod user;
pub mod permission;


use serde::{Deserialize, Serialize};
#[derive(Debug,Deserialize)]
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