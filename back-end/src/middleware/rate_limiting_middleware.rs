use axum::{
    extract::FromRequestParts,
    http::StatusCode,
    response::Response,
};
use std::{collections::HashMap, sync::Mutex, time::{Duration, Instant}};
use lazy_static::lazy_static;

lazy_static! {
    static ref RATE_LIMITS: Mutex<HashMap<String, (u64, Instant)>> = Mutex::new(HashMap::new());
}

const LIMIT: u64 = 10; 
const WINDOW: Duration = Duration::from_secs(10);

pub struct RateLimitingMiddleware;

impl<S> FromRequestParts<S> for RateLimitingMiddleware
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        req: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        let ip = req
            .headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");

        let mut rate_limits = RATE_LIMITS.lock().unwrap();
        let entry = rate_limits.entry(ip.to_string()).or_insert((0, Instant::now()));

        // Reset nếu vượt thời gian
        if entry.1.elapsed() > WINDOW {
            entry.0 = 0;
            entry.1 = Instant::now();
        }

        // Kiểm tra giới hạn
        if entry.0 >= LIMIT {
            return Err((StatusCode::TOO_MANY_REQUESTS, "Too Many Requests"));
        }

        entry.0 += 1;

        Ok(Self)
    }
}
