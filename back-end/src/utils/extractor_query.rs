use axum::{extract::{rejection::QueryRejection, FromRequest, Query, Request}, http::StatusCode};
use serde::de::DeserializeOwned;

pub struct QueryExtractor<T>(pub T);

impl<S, T> FromRequest<S> for QueryExtractor<T>
where
    Query<T>: FromRequest<S, Rejection = QueryRejection>,
    T: DeserializeOwned + Send + Sync,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let req = Request::from_parts(parts, body);

        match Query::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err((
                rejection.status(),
                format!("{}", rejection.body_text()),
            )),
        }
    }
}
