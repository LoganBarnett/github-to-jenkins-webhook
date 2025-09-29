use clap::Parser;
use std::fs;
use std::path::PathBuf;
use tracing::Level;

#[derive(Parser, Debug)]
#[clap(name = "github-jenkins-proxy")]
#[clap(
  about = "A secure proxy between GitHub webhooks and Jenkins",
  long_about = None,
)]
pub struct Args {
  #[clap(
    short = 's',
    long = "github-secret",
    env = "GITHUB_SECRET",
    help = "GitHub webhook secret for HMAC validation",
    conflicts_with = "github_secret_file"
  )]
  pub github_secret: Option<String>,

  #[clap(
    long = "github-secret-file",
    help = "Path to file containing GitHub webhook secret for HMAC validation",
    conflicts_with = "github_secret"
  )]
  pub github_secret_file: Option<PathBuf>,

  #[clap(
    short = 'j',
    long = "jenkins-url",
    env = "JENKINS_URL",
    help = "Jenkins server URL to forward webhooks to"
  )]
  pub jenkins_url: String,

  #[clap(
    short = 'H',
    long = "host",
    env = "HOST",
    default_value = "0.0.0.0",
    help = "Host address to bind the server to"
  )]
  pub host: String,

  #[clap(
    short = 'p',
    long = "port",
    env = "PORT",
    default_value = "8080",
    help = "Port to bind the server to"
  )]
  pub port: u16,

  #[clap(
    short = 'l',
    long = "log-level",
    env = "LOG_LEVEL",
    default_value = "info",
    help = "Log level (trace, debug, info, warn, error)"
  )]
  pub log_level: String,
}

impl Args {
  pub fn get_github_secret(&self) -> Result<String, String> {
    if let Some(secret) = &self.github_secret {
      Ok(secret.clone())
    } else if let Some(path) = &self.github_secret_file {
      fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|e| {
          format!(
            "Failed to read GitHub secret from file '{}': {}",
            path.display(),
            e,
          )
        })
    } else {
      Err(
        "Either --github-secret or --github-secret-file must be provided"
          .to_string(),
      )
    }
  }

  pub fn get_log_level(&self) -> Result<Level, String> {
    match self.log_level.to_lowercase().as_str() {
      "trace" => Ok(Level::TRACE),
      "debug" => Ok(Level::DEBUG),
      "info" => Ok(Level::INFO),
      "warn" => Ok(Level::WARN),
      "error" => Ok(Level::ERROR),
      _ => Err(format!(
        "Invalid log level '{}'. Valid options are: trace, debug, info, warn, error",
        self.log_level
      )),
    }
  }
}
