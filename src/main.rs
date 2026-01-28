mod cli;
mod config;
mod env;
mod memory;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Commands};
use config::{Config, LocalConfig};
use env::{filter_existing, load_env_files};
use memory::Memory;
use run::exec;

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
        eprintln!("Error: couldn't get current directory");
        return ExitCode::FAILURE;
    };

    let mem = Memory::load();
    if let Some(files) = mem.get(cwd) {
        for f in files {
            println!("{f}");
        }
        ExitCode::SUCCESS
    } else {
        println!("no env files remembered for this directory");
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

    let env_files = if cli.env_files.is_empty() && memory_enabled {
        let mem = Memory::load();
        cwd.and_then(|d| mem.get(d).cloned())
            .map(|files| filter_existing(&files))
            .unwrap_or_default()
    } else if let Some(local) = local {
        local.expand_aliases(&cli.env_files)
    } else {
        cli.env_files.clone()
    };

    if env_files.is_empty() {
        eprintln!("Error: No env files specified and none in memory");
        return ExitCode::FAILURE;
    }

    if cli.command.is_empty() {
        eprintln!("Error: No command specified");
        return ExitCode::FAILURE;
    }

    let env_vars = match load_env_files(&env_files) {
        Ok(vars) => vars,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    if memory_enabled
        && let Some(cwd) = cwd
    {
        let mut mem = Memory::load();
        mem.record(cwd, &env_files, config.memory.max_entries);
        let _ = mem.save();
    }

    exec(&cli.command, env_vars)
}
