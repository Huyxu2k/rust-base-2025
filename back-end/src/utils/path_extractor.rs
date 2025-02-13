use axum::{extract::{rejection::PathRejection, FromRequestParts, Path}, http::{request::Parts, StatusCode}};
use serde::de::DeserializeOwned;


pub struct PathExtrator<T>(pub T);

impl<S, T> FromRequestParts<S> for PathExtrator<T> 
where 
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    //TODO
    type Rejection = (StatusCode, String);
    
    async fn from_request_parts(parts: &mut Parts,state: &S,) ->Result<Self,Self::Rejection>{
        match Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body)= match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        (StatusCode::BAD_REQUEST,inner.to_string())
                    },
                    PathRejection::MissingPathParams(error) => {
                        (StatusCode::INTERNAL_SERVER_ERROR,error.to_string())
                    },
                    _ => (StatusCode::INTERNAL_SERVER_ERROR,format!("Unhandled path rejection: {rejection}")),
                };
                Err((status,body))
            },
        }
    }
}

