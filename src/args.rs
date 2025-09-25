use clap::Parser;
use std::fs;
use std::path::PathBuf;
use clap_verbosity_flag::Verbosity;

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

  #[command(flatten)]
  pub verbosity: Verbosity,
}

impl Args {
  pub fn get_github_secret(&self) -> Result<String, String> {
    if let Some(secret) = &self.github_secret {
      Ok(secret.clone())
    } else if let Some(path) = &self.github_secret_file {
      fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!(
          "Failed to read GitHub secret from file '{}': {}",
          path.display(),
          e,
        ))
    } else {
      Err("Either --github-secret or --github-secret-file must be provided".to_string())
    }
  }
}
