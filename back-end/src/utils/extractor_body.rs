use axum::{extract::{rejection::JsonRejection, FromRequest, Request}, http::StatusCode, Json};
use serde::de::DeserializeOwned;



pub struct JsonExtractor<T>(pub T);


impl <S, T> FromRequest<S> for JsonExtractor<T>
where 
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    T: DeserializeOwned + Send + Sync,
    S: Send+ Sync
{
    
    type Rejection=(StatusCode, String);

    async fn from_request(req:Request,state: &S,) ->Result<Self,Self::Rejection>{
        let (parts, body)= req.into_parts();


        let req= Request::from_parts(parts, body);

        match Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err((
                rejection.status(),
                format!("{}",rejection.body_text())
            )),
        }
    }
}