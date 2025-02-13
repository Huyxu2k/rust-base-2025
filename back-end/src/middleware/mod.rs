use crate::AppState;

pub(crate) mod layer;
//pub mod rate_limiting_middleware;


use axum::{
    extract::Request, response::Response
};
use tower::{Service, Layer};
use std::task::{Context, Poll};
use futures_util::future::BoxFuture;


pub const AUTHORIZATION_HEADER: &str= "Authorization";
pub const BEARER: &str= "Bearer ";

pub trait THandler {
    fn handle_request<B>(req: Request<B>, state: &AppState) -> Result<Request<B>,Response>;
}

#[derive(Clone)]
pub struct TLayer<T> {
    state: AppState,
    _marker: std::marker::PhantomData<T>,
}

impl<T> TLayer<T> {
    pub fn new(state: AppState) -> Self {
        Self { 
            state, 
            _marker: std::marker::PhantomData 
        }
    }
}

impl<S, T> Layer<S> for TLayer<T> {
    type Service = TMiddleware<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        TMiddleware { 
            inner, 
            state: self.state.clone(),
            _marker: std::marker::PhantomData
        }
    }
}

#[derive(Clone)]
pub struct TMiddleware<S, T> {
    inner: S,
    state: AppState,
    _marker: std::marker::PhantomData<T>,
}

impl<S, T> Service<Request> for TMiddleware<S, T>
where
    S: Service<Request, Response= Response>+ Send + 'static,
    S::Future: Send+'static,
    T: THandler,  
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response,Self::Error>>;//S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let result = T::handle_request(req, &self.state);
        match result {
            Ok(modified_req) =>{
                let future= self.inner.call(modified_req);
                Box::pin(async move{
                    let response: Response= future.await?;
                    Ok(response)
                })
            },
            Err(response) => Box::pin(async { Ok(response) }),
        }
    }
}
