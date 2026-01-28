mod cli;
mod config;
mod env;
mod memory;
mod run;
mod watch;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use colored::Colorize;

use cli::{Cli, Commands};
use config::{Config, LocalConfig};
use env::{filter_existing, load_env_files};
use memory::Memory;
use run::exec;
use watch::watch_and_run;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let config = Config::load();
    let cwd = std::env::current_dir().ok();
    let local = cwd.as_ref().and_then(|d| LocalConfig::load(d));

    match cli.subcommand {
        Some(Commands::Ls) => cmd_ls(cwd.as_ref()),
        None => cmd_run(&cli, &config, local.as_ref(), cwd.as_ref()),
    }
}

fn cmd_ls(cwd: Option<&PathBuf>) -> ExitCode {
    let Some(cwd) = cwd else {
        eprintln!("{} couldn't get current directory", "error:".red().bold());
        return ExitCode::FAILURE;
    };

    let mem = Memory::load();
    if let Some(files) = mem.get(cwd) {
        for f in files {
            println!("{f}");
        }
        ExitCode::SUCCESS
    } else {
        println!("{}", "no env files remembered for this directory".dimmed());
        ExitCode::SUCCESS
    }
}

fn cmd_run(
    cli: &Cli,
    config: &Config,
    local: Option<&LocalConfig>,
    cwd: Option<&PathBuf>,
) -> ExitCode {
    let memory_enabled = local
        .and_then(|l| l.memory.enabled)
        .unwrap_or(config.memory.enabled);

    let (raw_files, env_files) = if cli.env_files.is_empty() && memory_enabled {
        let mem = Memory::load();
        let recalled = cwd.and_then(|d| mem.get(d).cloned()).unwrap_or_default();
        let expanded = local.map_or_else(|| recalled.clone(), |l| l.expand_aliases(&recalled));
        let filtered = filter_existing(&expanded);
        (recalled, filtered)
    } else {
        let expanded = local.map_or_else(
            || cli.env_files.clone(),
            |l| l.expand_aliases(&cli.env_files),
        );
        (cli.env_files.clone(), expanded)
    };

    if env_files.is_empty() {
        eprintln!(
            "{} no env files specified and none in memory",
            "error:".red().bold()
        );
        return ExitCode::FAILURE;
    }

    if cli.command.is_empty() {
        eprintln!("{} no command specified", "error:".red().bold());
        return ExitCode::FAILURE;
    }

    let files_to_remember: Vec<_> = raw_files
        .iter()
        .filter(|f| !f.starts_with('@'))
        .cloned()
        .collect();
    if memory_enabled
        && !files_to_remember.is_empty()
        && let Some(cwd) = cwd
    {
        let mut mem = Memory::load();
        mem.record(cwd, &files_to_remember, config.memory.max_entries);
        let _ = mem.save();
    }

    if cli.watch {
        watch_and_run(&env_files, &cli.command);
    }

    let env_vars = match load_env_files(&env_files) {
        Ok(vars) => vars,
        Err(e) => {
            eprintln!("{} {e}", "error:".red().bold());
            return ExitCode::FAILURE;
        }
    };

    exec(&cli.command, env_vars)
}
