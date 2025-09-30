mod args;
mod datetime_agnostic;
mod error;
mod github_types;
mod webhook;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use clap::Parser;
use tracing::{info, warn};

use crate::args::Args;
use crate::error::ProxyError;
use crate::webhook::handle_webhook;

#[tokio::main]
async fn main() -> Result<(), ProxyError> {
  let args = Args::parse();

  let log_level = args
    .get_log_level()
    .map_err(|e| ProxyError::Configuration(e))?;

  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .with_max_level(log_level)
    .init();

  let bind_address = format!("{}:{}", args.host, args.port);

  info!(
    "Starting GitHub to Jenkins webhook proxy on {}",
    bind_address
  );
  info!("Forwarding webhooks to: {}", args.jenkins_url);

  let github_secret = args
    .get_github_secret()
    .map_err(|e| ProxyError::Configuration(e))?;

  let app_state = web::Data::new(AppState {
    jenkins_url: args.jenkins_url,
    github_secret,
  });

  HttpServer::new(move || {
    App::new()
      .app_data(app_state.clone())
      .wrap(middleware::Logger::default())
      .service(
        web::resource("/github-webhook/").route(web::post().to(handle_webhook)),
      )
      .service(web::resource("/").route(web::get().to(health_check)))
      .default_service(web::route().to(not_found))
  })
  .bind(bind_address)?
  .run()
  .await
  .map_err(ProxyError::from)
}

#[derive(Clone)]
pub struct AppState {
  pub jenkins_url: String,
  pub github_secret: String,
}

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().body("GitHub to Jenkins Webhook Proxy is running")
}

async fn not_found(req: HttpRequest) -> HttpResponse {
  warn!(
    "404 Not Found: {} {} from {:?}",
    req.method(),
    req.path(),
    req.connection_info().peer_addr()
  );
  HttpResponse::NotFound().body("Not Found")
}
