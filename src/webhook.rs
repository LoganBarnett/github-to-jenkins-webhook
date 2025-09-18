use actix_web::{web, HttpRequest, HttpResponse};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tracing::{debug, error, info};

use crate::error::ProxyError;
use crate::AppState;

type HmacSha256 = Hmac<Sha256>;

const GITHUB_SIGNATURE_HEADER: &str = "X-Hub-Signature-256";

pub async fn handle_webhook(
  req: HttpRequest,
  body: web::Bytes,
  state: web::Data<AppState>,
) -> Result<HttpResponse, ProxyError> {
  let signature_header = req
    .headers()
    .get(GITHUB_SIGNATURE_HEADER)
    .ok_or(ProxyError::MissingSignature)?;

  let signature = signature_header
    .to_str()
    .map_err(|_| ProxyError::InvalidHeader("Invalid signature header".to_string()))?;

  if !verify_signature(&body, signature, &state.github_secret)? {
    error!("Invalid signature from GitHub webhook");
    return Err(ProxyError::InvalidSignature);
  }

  info!("Valid GitHub webhook received, forwarding to Jenkins");

  forward_to_jenkins(&req, &body, &state.jenkins_url).await
}

fn verify_signature(payload: &[u8], signature: &str, secret: &str) -> Result<bool, ProxyError> {
  if !signature.starts_with("sha256=") {
    return Ok(false);
  }

  let signature = &signature[7..];

  let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
    .map_err(|_| ProxyError::HmacComputation)?;

  mac.update(payload);

  let result = mac.finalize();
  let expected = hex::encode(result.into_bytes());

  debug!("Expected signature: {}", expected);
  debug!("Received signature: {}", signature);

  Ok(expected == signature)
}

async fn forward_to_jenkins(
  original_req: &HttpRequest,
  body: &web::Bytes,
  jenkins_url: &str,
) -> Result<HttpResponse, ProxyError> {
  let client = reqwest::Client::new();

  let mut req_builder = client
    .post(jenkins_url)
    .body(body.to_vec());

  for (header_name, header_value) in original_req.headers() {
    if header_name == GITHUB_SIGNATURE_HEADER {
      continue;
    }

    if let Ok(name) = header_name.to_string().parse::<reqwest::header::HeaderName>() {
      if let Ok(value) = header_value.to_str() {
        if let Ok(value) = value.parse::<reqwest::header::HeaderValue>() {
          req_builder = req_builder.header(name, value);
        }
      }
    }
  }

  let response = req_builder.send().await?;

  let status = response.status();
  let body = response.bytes().await?;

  info!(
    "Forwarded webhook to Jenkins. Response status: {}",
    status
  );

  Ok(HttpResponse::build(actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap())
     .body(body))
}
