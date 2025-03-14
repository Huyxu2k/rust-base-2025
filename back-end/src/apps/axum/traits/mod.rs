use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};

// pub mod params_body;
// pub mod params_path;
// pub mod params_query;


#[derive(Debug,Deserialize,Serialize)]
pub struct TResponse<T> {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub status: String,
    pub message: String,
    pub data: Option<T>,
    pub errors: Option<String>,
    pub pagination: Option<PaginationResponse>
}

impl <T: Serialize> IntoResponse for  TResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let body=Json(&self);
        (self.status_code,body).into_response()
    }
}

#[derive(Debug,Deserialize,Serialize)]
pub struct PaginationResponse{
    pub current_page: usize,
    pub per_page: usize,
    pub total_pages: usize,
    pub total_items: usize
}

pub trait ValidateRequired{
    fn validate_requierd(&self)->Result<(),Response>;
}

pub fn validate_requierd<T: Serialize>(data: &T)-> Result<(), TResponse<()>>{
    let serialized= serde_json::to_value(data).unwrap();

    let mut missing_fields= Vec::new();

    if let serde_json::Value::Object(map)= serialized{
        for (key, value) in map{
            if value.is_null(){
                missing_fields.push(key);
            }
        }
    }
    if !missing_fields.is_empty(){
        return Err(
            TResponse{
                status_code: StatusCode::BAD_REQUEST,
                status: "error".to_owned(),
                message: format!("Requied field: {:?}",missing_fields),
                data: None,
                errors: None,
                pagination: None,
            }
        )
    }
    
    Ok(())
}
