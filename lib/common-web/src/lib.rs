use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, http::header, HttpResponse, ResponseError};
use serde::Serialize;
use std::future::{ready, Ready};
use std::task::{Context, Poll};
use thiserror::Error;

pub fn cors_permissive() -> Cors {
    Cors::permissive()
}

#[derive(Debug, Error, Serialize)]
#[serde(tag = "error", content = "message")]
pub enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal server error")]
    Internal,
}
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().json(self),
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(self),
            ApiError::Internal => HttpResponse::InternalServerError().json(self),
        }
    }
}

// 简易 Request-Id 中间件（演示）
pub struct RequestId;
impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for RequestId
where
    S: actix_web::dev::Service<
            ServiceRequest,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = RequestIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddleware(service)))
    }
}
pub struct RequestIdMiddleware<S>(S);
impl<S, B> actix_web::dev::Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: actix_web::dev::Service<
            ServiceRequest,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = S::Future;
    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let id = uuid::Uuid::new_v4().to_string();
        req.headers_mut().insert(
            header::HeaderName::from_static("x-request-id"),
            header::HeaderValue::from_str(&id).unwrap(),
        );
        self.0.call(req)
    }
}
