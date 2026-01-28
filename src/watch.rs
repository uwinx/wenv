use colored::Colorize;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::env::load_env_files;

const DEBOUNCE_MS: u64 = 200;

pub fn watch_and_run(env_files: &[String], command: &[String]) -> ! {
    let Some(env_vars) = try_load(env_files) else {
        std::process::exit(1);
    };

    let mut child = Some(spawn(command, &env_vars));

    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_secs(1)),
    )
    .expect("failed to create watcher");

    for file in env_files {
        if let Err(e) = watcher.watch(Path::new(file), RecursiveMode::NonRecursive) {
            eprintln!("{} couldn't watch {file}: {e}", "warning:".yellow().bold());
        }
    }

    println!("\n{}", "[watching]".dimmed());

    let mut last_run = Instant::now();

    loop {
        match rx.recv() {
            Ok(event) => {
                if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    let now = Instant::now();
                    if now.duration_since(last_run) > Duration::from_millis(DEBOUNCE_MS) {
                        last_run = now;
                        println!("\n{}", "[restarting]".cyan());

                        if let Some(mut c) = child.take() {
                            let _ = c.kill();
                            let _ = c.wait();
                        }

                        if let Some(env_vars) = try_load(env_files) {
                            child = Some(spawn(command, &env_vars));
                        }

                        println!("\n{}", "[watching]".dimmed());
                    }
                }
            }
            Err(e) => {
                eprintln!("{} {e}", "watch error:".red().bold());
                if let Some(mut c) = child.take() {
                    let _ = c.kill();
                    let _ = c.wait();
                }
                std::process::exit(1);
            }
        }
    }
}

fn spawn(command: &[String], env_vars: &HashMap<String, String>) -> Child {
    let (program, args) = command.split_first().expect("empty command");

    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.envs(env_vars);

    cmd.spawn().expect("failed to spawn command")
}

fn try_load(env_files: &[String]) -> Option<HashMap<String, String>> {
    match load_env_files(env_files) {
        Ok(vars) => Some(vars),
        Err(e) => {
            eprintln!("{} {e}", "error:".red().bold());
            None
        }
    }
}
