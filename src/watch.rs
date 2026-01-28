use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::env::load_env_files;
use crate::run::exec;

const DEBOUNCE_MS: u64 = 200;

pub fn watch_and_run(env_files: &[String], command: &[String]) -> ! {
    run_once(env_files, command);

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
            eprintln!("warning: couldn't watch {file}: {e}");
        }
    }

    println!("\n[watching for changes...]");

    let mut last_run = Instant::now();

    loop {
        match rx.recv() {
            Ok(event) => {
                if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    let now = Instant::now();
                    if now.duration_since(last_run) > Duration::from_millis(DEBOUNCE_MS) {
                        last_run = now;
                        println!("\n[rerunning...]");
                        run_once(env_files, command);
                        println!("\n[watching for changes...]");
                    }
                }
            }
            Err(e) => {
                eprintln!("watch error: {e}");
                std::process::exit(1);
            }
        }
    }
}

fn run_once(env_files: &[String], command: &[String]) {
    let env_vars = match load_env_files(env_files) {
        Ok(vars) => vars,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    exec(command, env_vars);
}
