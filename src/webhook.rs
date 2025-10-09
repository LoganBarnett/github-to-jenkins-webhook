use actix_web::{web, HttpRequest, HttpResponse};
use hmac::{Hmac, Mac};
use serde::de::DeserializeOwned;
use sha2::Sha256;
use tracing::{debug, error, info, warn};
use url::Url;

use crate::error::ProxyError;
use crate::github_types::GitHubWebhookPayload;
use crate::AppState;

type HmacSha256 = Hmac<Sha256>;

const GITHUB_SIGNATURE_HEADER: &str = "X-Hub-Signature-256";
const GITHUB_EVENT_HEADER: &str = "X-GitHub-Event";
const MAX_PAYLOAD_SIZE: usize = 25 * 1024 * 1024;

pub async fn handle_webhook(
  req: HttpRequest,
  body: web::Bytes,
  state: web::Data<AppState>,
) -> Result<HttpResponse, ProxyError> {
  if body.len() > MAX_PAYLOAD_SIZE {
    error!("Payload size {} exceeds maximum allowed size", body.len());
    return Err(ProxyError::PayloadTooLarge);
  }

  let event_type = req
    .headers()
    .get(GITHUB_EVENT_HEADER)
    .and_then(|h| h.to_str().ok())
    .unwrap_or("unknown");

  info!("Received GitHub webhook event: {}", event_type);

  let signature_header = req
    .headers()
    .get(GITHUB_SIGNATURE_HEADER)
    .ok_or(ProxyError::MissingSignature)?;

  let signature = signature_header.to_str().map_err(|_| {
    ProxyError::InvalidHeader("Invalid signature header".to_string())
  })?;

  if !verify_signature(&body, signature, &state.github_secret)? {
    error!("Invalid signature from GitHub webhook");
    return Err(ProxyError::InvalidSignature);
  }

  debug!(
    "Webhook payload (first 1000 chars): {}",
    String::from_utf8_lossy(&body[..body.len().min(1000)]),
  );

  let payload: GitHubWebhookPayload =
    parse_payload_from_header(event_type, &body)?;

  if !payload.validate_required_fields() {
    error!("GitHub webhook payload missing required fields");
    return Err(ProxyError::InvalidPayload(
      "Payload missing required fields".to_string(),
    ));
  }

  info!("Valid GitHub webhook payload received");

  if !validate_jenkins_url(&state.jenkins_url)? {
    error!("Invalid Jenkins URL configuration");
    return Err(ProxyError::InvalidJenkinsUrl);
  }

  let jenkins_webhook_path = construct_jenkins_url(&state.jenkins_url)?;

  info!("Forwarding to Jenkins: {}", jenkins_webhook_path);

  forward_to_jenkins(&req, &body, &jenkins_webhook_path).await
}

fn from_slice_with_path<T: DeserializeOwned>(
  bytes: &[u8],
) -> Result<T, ProxyError> {
  let mut de = serde_json::Deserializer::from_slice(bytes);
  match serde_path_to_error::deserialize(&mut de) {
    Ok(v) => Ok(v),
    Err(e) => Err(ProxyError::SerdePath {
      path: e.path().to_string(),
      source: e.into_inner(),
    }),
  }
}

fn parse_payload_from_header(
  event_type: &str,
  body: &web::Bytes,
) -> Result<GitHubWebhookPayload, ProxyError> {
  match event_type {
    "push" => Ok(GitHubWebhookPayload::Push(from_slice_with_path(&body)?)),
    "pull_request" => Ok(GitHubWebhookPayload::PullRequest(
      from_slice_with_path(&body)?,
    )),
    "issues" => Ok(GitHubWebhookPayload::Issues(from_slice_with_path(&body)?)),
    "issue_comment" => Ok(GitHubWebhookPayload::IssueComment(
      from_slice_with_path(&body)?,
    )),
    "create" => Ok(GitHubWebhookPayload::Create(from_slice_with_path(&body)?)),
    "delete" => Ok(GitHubWebhookPayload::Delete(from_slice_with_path(&body)?)),
    "fork" => Ok(GitHubWebhookPayload::Fork(from_slice_with_path(&body)?)),
    "release" => {
      Ok(GitHubWebhookPayload::Release(from_slice_with_path(&body)?))
    }
    _ => Err(ProxyError::InvalidPayload(format!(
      "Event type `{}' not supported.",
      event_type,
    ))),
  }
  .map_err(|e| {
    error!("Failed to parse GitHub webhook payload: {}", e);
    ProxyError::InvalidPayload(format!("Invalid GitHub webhook payload: {}", e))
  })
}

fn verify_signature(
  payload: &[u8],
  signature: &str,
  secret: &str,
) -> Result<bool, ProxyError> {
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

fn validate_jenkins_url(jenkins_url: &str) -> Result<bool, ProxyError> {
  let url =
    Url::parse(jenkins_url).map_err(|_| ProxyError::InvalidJenkinsUrl)?;

  if url.scheme() != "http" && url.scheme() != "https" {
    warn!("Jenkins URL has invalid scheme: {}", url.scheme());
    return Ok(false);
  }

  if url.host_str().is_none() {
    warn!("Jenkins URL has no host");
    return Ok(false);
  }

  let path = url.path();
  if path.contains("..") || path.contains("//") {
    warn!("Jenkins URL contains suspicious path traversal patterns");
    return Ok(false);
  }

  if url.query().is_some() || url.fragment().is_some() {
    warn!("Jenkins URL should not contain query parameters or fragments");
    return Ok(false);
  }

  Ok(true)
}

fn construct_jenkins_url(base_url: &str) -> Result<String, ProxyError> {
  let mut url =
    Url::parse(base_url).map_err(|_| ProxyError::InvalidJenkinsUrl)?;

  let path = url.path().to_string();

  let new_path = if !path.ends_with('/') {
    format!("{}/", path)
  } else {
    path.clone()
  };

  if !new_path.contains("/github-webhook/") && !new_path.contains("/ghprbhook/")
  {
    url.set_path(&format!("{}github-webhook/", new_path));
  } else {
    url.set_path(&new_path);
  }

  Ok(url.to_string())
}

async fn forward_to_jenkins(
  original_req: &HttpRequest,
  body: &web::Bytes,
  jenkins_url: &str,
) -> Result<HttpResponse, ProxyError> {
  let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(30))
    .build()
    .map_err(|e| ProxyError::ForwardRequest(e))?;

  let mut req_builder = client.post(jenkins_url).body(body.to_vec());

  for (header_name, header_value) in original_req.headers() {
    // if header_name == GITHUB_SIGNATURE_HEADER {
    //   continue;
    // }

    let header_name_str = header_name.as_str();
    if header_name_str.to_lowercase().starts_with("x-github-")
      || header_name_str.to_lowercase().starts_with("x-hub-")
      || header_name_str.to_lowercase().starts_with("x-forwarded-")
      || header_name_str.to_lowercase() == "host"
      || header_name_str.to_lowercase() == "accept"
      || header_name_str.to_lowercase() == "content-type"
    {
      debug!(
        "Passing header - {}: {:?}",
        header_name,
        header_value.to_str()
      );
      req_builder = req_builder.header(header_name.clone(), header_value);
    } else {
      debug!(
        "Dropping header - {}: {:?}",
        header_name,
        header_value.to_str()
      );
    }
  }

  let response = req_builder.send().await?;

  let status = response.status();
  let body = response.bytes().await?;

  info!("Forwarded webhook to Jenkins. Response status: {}", status,);

  Ok(
    HttpResponse::build(
      actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap(),
    )
    .body(body),
  )
}
