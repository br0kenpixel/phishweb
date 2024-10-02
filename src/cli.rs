use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    /// IP address to bind to
    #[arg(long, value_name = "IP")]
    pub ip: Option<String>,

    /// Port to bind to
    #[arg(long, value_name = "PORT")]
    pub port: Option<u16>,

    /// Path to webroot directory
    #[arg(long, value_name = "PATH")]
    pub webroot: Option<PathBuf>,

    /// File to store captured credentials in
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,
}
