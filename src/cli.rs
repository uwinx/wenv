use clap::{Parser, Subcommand};

const AFTER_HELP: &str = "\
wenv remembers which env files you use per directory.
run without env files to reuse the last ones.
(aliases aren't memorized - they're already shortcuts)

aliases in .wenv.toml:
  [aliases]
  dev = [\".env\", \".env.dev\"]

  then: wenv @dev -- cmd

config:
  mac:   ~/Library/Application Support/wenv/
  linux: ~/.config/wenv/";

#[derive(Parser)]
#[command(name = "wenv")]
#[command(about = "load env files and run stuff")]
#[command(after_help = AFTER_HELP)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[arg(help = "env files to load (uses memory if empty)")]
    pub env_files: Vec<String>,

    #[arg(short, long, help = "rerun on env file changes")]
    pub watch: bool,

    #[arg(last = true, help = "command to run")]
    pub command: Vec<String>,

    #[command(subcommand)]
    pub subcommand: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// show remembered env files for current directory
    Ls,
}
