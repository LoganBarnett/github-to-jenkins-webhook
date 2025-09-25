use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProxyError {
  #[error("Failed to bind server: {0}")]
  ServerBind(#[from] std::io::Error),

  #[error("Invalid signature")]
  InvalidSignature,

  #[error("Missing signature header")]
  MissingSignature,

  #[error("Failed to compute HMAC")]
  HmacComputation,

  #[error("Failed to forward request: {0}")]
  ForwardRequest(#[from] reqwest::Error),

  #[error("Failed to read request body")]
  ReadBody,

  #[error("Invalid header value: {0}")]
  InvalidHeader(String),

  #[error("Server error: {0}")]
  ServerError(String),

  #[error("Invalid payload: {0}")]
  InvalidPayload(String),

  #[error("Payload size exceeds maximum allowed size")]
  PayloadTooLarge,

  #[error("Invalid Jenkins URL")]
  InvalidJenkinsUrl,

  #[error("Configuration error: {0}")]
  Configuration(String),
}

impl ResponseError for ProxyError {
  fn error_response(&self) -> HttpResponse {
    match self {
      ProxyError::InvalidSignature | ProxyError::MissingSignature => {
        HttpResponse::Unauthorized().body(self.to_string())
      }
      ProxyError::ForwardRequest(_) => {
        HttpResponse::BadGateway().body("Failed to forward request to Jenkins")
      }
      ProxyError::ReadBody
        | ProxyError::InvalidHeader(_)
        | ProxyError::InvalidPayload(_)
        | ProxyError::PayloadTooLarge => {
        HttpResponse::BadRequest().body(self.to_string())
      }
      ProxyError::InvalidJenkinsUrl | ProxyError::Configuration(_) => {
        HttpResponse::InternalServerError().body(self.to_string())
      }
      _ => HttpResponse::InternalServerError().body("Internal server error"),
    }
  }

  fn status_code(&self) -> StatusCode {
    match self {
      ProxyError::InvalidSignature | ProxyError::MissingSignature => {
        StatusCode::UNAUTHORIZED
      }
      ProxyError::ForwardRequest(_) => StatusCode::BAD_GATEWAY,
      ProxyError::ReadBody | ProxyError::InvalidHeader(_) | ProxyError::InvalidPayload(_) | ProxyError::PayloadTooLarge => StatusCode::BAD_REQUEST,
      ProxyError::InvalidJenkinsUrl | ProxyError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}
