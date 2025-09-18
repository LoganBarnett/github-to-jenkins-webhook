use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "github-jenkins-proxy")]
#[clap(about = "A secure proxy between GitHub webhooks and Jenkins", long_about = None)]
pub struct Args {
    #[clap(
        short = 's',
        long = "github-secret",
        env = "GITHUB_SECRET",
        help = "GitHub webhook secret for HMAC validation"
    )]
    pub github_secret: String,

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
}