use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!(
            "Usage: {} <env_file1> [env_file2 ...] -- <command> [args...]",
            args[0]
        );
        exit(1);
    }

    let separator_pos = args.iter().position(|arg| arg == "--").unwrap_or_else(|| {
        eprintln!("Error: Missing '--' separator");
        exit(1);
    });

    let env_files = &args[1..separator_pos];
    let command_args = &args[separator_pos + 1..];

    if command_args.is_empty() {
        eprintln!("Error: No command specified after '--'");
        exit(1);
    }

    let mut env_vars = HashMap::new();

    for env_file in env_files {
        match fs::read_to_string(env_file) {
            Ok(content) => {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }

                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim().trim_matches('"').trim_matches('\'');
                        env_vars.insert(key.to_string(), value.to_string());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading {}: {}", env_file, e);
                exit(1);
            }
        }
    }

    let mut cmd = Command::new(&command_args[0]);
    cmd.args(&command_args[1..]);

    for (key, value) in env_vars {
        cmd.env(key, value);
    }

    match cmd.status() {
        Ok(status) => {
            exit(status.code().unwrap_or(1));
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
            exit(1);
        }
    }
}
