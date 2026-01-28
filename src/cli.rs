use clap::Parser;

#[derive(Parser)]
#[command(name = "wenv")]
#[command(about = "Run commands with environment variables from .env files")]
pub struct Args {
    pub env_files: Vec<String>,

    #[arg(last = true, required = true)]
    pub command: Vec<String>,
}
