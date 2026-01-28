mod cli;
mod config;
mod env;
mod memory;
mod run;

use std::process::ExitCode;

use clap::Parser;

use cli::Args;
use config::{Config, LocalConfig};
use env::{filter_existing, load_env_files};
use memory::Memory;
use run::exec;

fn main() -> ExitCode {
    let args = Args::parse();
    let config = Config::load();

    let cwd = std::env::current_dir().ok();
    let local = cwd.as_ref().and_then(|d| LocalConfig::load(d));

    let memory_enabled = local
        .as_ref()
        .and_then(|l| l.memory.enabled)
        .unwrap_or(config.memory.enabled);

    let env_files = if args.env_files.is_empty() && memory_enabled {
        let mem = Memory::load();
        cwd.as_ref()
            .and_then(|d| mem.get(d).cloned())
            .map(|files| filter_existing(&files))
            .unwrap_or_default()
    } else {
        args.env_files.clone()
    };

    if env_files.is_empty() {
        // todo(uwinx): too harsh (???), consider warning here
        eprintln!("Error: No env files specified and none in memory");
        return ExitCode::FAILURE;
    }

    let env_vars = match load_env_files(&env_files) {
        Ok(vars) => vars,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    if memory_enabled {
        if let Some(ref cwd) = cwd {
            let mut mem = Memory::load();
            mem.record(cwd, &env_files, config.memory.max_entries);
            let _ = mem.save();
        }
    }

    exec(&args.command, env_vars)
}
