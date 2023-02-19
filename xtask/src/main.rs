use anyhow::{Context, Result};
use log::info;
use pretty_env_logger::env_logger;
use rayon::prelude::*;
use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};
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
    let mut command = Command::new("python")
        .arg(script)
        .stdin(Stdio::piped())
        .spawn()
        .with_context(|| ScriptError::CommandError(script.to_owned()))?;

    {
        let stdin = command.stdin.as_mut().ok_or_else(|| {
            ScriptError::CommandError(format!("Failed to open stdin for {}", script))
        })?;

        let buf = fs::read(input_file)
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
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let input_files = vec!["input.txt", "input.txt"];
    let scripts = vec!["entity.py", "triagram.py"];

    scripts
        .iter()
        .zip(input_files.iter()) // 'Zips up' two iterators into a single iterator of pairs.
        .par_bridge() // Creates a bridge from this type to a `ParallelIterator`.
        .map(|(script, input_file)| {
            let span = info_span!("Script", script = script);
            let _guard = span.enter();
            execute_script(script, input_file)
        })
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| ScriptError::CommandError("Failed to execute scripts".to_owned()))?;

    info!("Execution completed successfully");

    Ok(())
}
