use serde::Deserialize;

pub mod params_body;
pub mod params_path;
pub mod params_query;



#[derive(Debug,Deserialize)]
pub struct Pagination{
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