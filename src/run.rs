use std::collections::HashMap;
use std::process::{Command, ExitCode};

pub fn exec(command: &[String], env_vars: HashMap<String, String>) -> ExitCode {
    let Some((program, args)) = command.split_first() else {
        eprintln!("Error: No command specified");
        return ExitCode::FAILURE;
    };

    let mut cmd = Command::new(program);
    cmd.args(args);

    for (key, value) in env_vars {
        cmd.env(key, value);
    }

    match cmd.status() {
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(e) => {
            eprintln!("Error executing command: {}", e);
            ExitCode::FAILURE
        }
    }
}
