use anyhow::{Context, Result};
use log::info;
use pretty_env_logger::env_logger::Env;
use pretty_env_logger::*;
use rayon::prelude::*;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use thiserror::Error;
use tracing::{info_span, instrument};

#[derive(Error, Debug)]
enum ScriptError {
    #[error("Failed to execute command: {0}")]
    CommandError(String),
    #[error("Failed to open file: {0}")]
    FileOpenError(String),
    #[error("Failed to read file: {0}")]
    FileReadError(String),
    #[error("Failed to write to stdin: {0}")]
    WriteError(String),
    #[error("Failed to wait for script: {0}")]
    WaitError(String),
}

fn execute_script(script: &str, input_file: &str) -> Result<()> {
    let mut command = Command::new("python3")
        .arg(script)
        .stdin(Stdio::piped())
        .spawn()
        .with_context(|| ScriptError::CommandError(script.to_owned()))?;

    {
        let stdin = command.stdin.as_mut().ok_or_else(|| {
            ScriptError::CommandError(format!("Failed to open stdin for {}", script))
        })?;

        let mut buf = fs::read(input_file)
            .with_context(|| ScriptError::FileOpenError(input_file.to_owned()))?;

        stdin
            .write_all(&buf)
            .with_context(|| ScriptError::WriteError(script.to_owned()))?;
    }

    command
        .wait()
        .with_context(|| ScriptError::WaitError(script.to_owned()))?;

    Ok(())
}

#[instrument]
fn main() -> Result<()> {
    let var = &std::env::var("RUST_LOG").unwrap_or_default();
    let env: Env = Env::new().filter(var);
    // env_logger::init_from_env(env);
    // let env: Env = Env::new().filter("MY_LOG").write_style("MY_LOG_STYLE");
    env_logger::init_from_env(env);

    let input_file = "input.txt";

    let script_closure = Box::new(|&script| {
        let span = info_span!("ENTITY");
        let _guard = span.enter();
        execute_script(script, input_file)
    });

    vec!["entity.py", "triagram.py"]
        .par_iter()
        .map(script_closure)
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| ScriptError::CommandError("Failed to execute scripts".to_owned()))?;

    info!("Execution completed successfully");

    Ok(())
}
