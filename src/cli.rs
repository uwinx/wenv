use clap::Parser;

const AFTER_HELP: &str = "\
wenv remembers which env files you use per directory.
run without env files to reuse the last ones.

config:
  mac:   ~/Library/Application Support/wenv/
  linux: ~/.config/wenv/

opt out per project with .wenv.toml:
  [memory]
  enabled = false";

#[derive(Parser)]
#[command(name = "wenv")]
#[command(about = "load env files and run stuff")]
#[command(after_help = AFTER_HELP)]
pub struct Args {
    #[arg(help = "env files to load (uses memory if empty)")]
    pub env_files: Vec<String>,

    #[arg(last = true, required = true, help = "command to run")]
    pub command: Vec<String>,
}
