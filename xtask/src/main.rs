use anyhow::{Context, Result};
use log::{error, info};
use miette::Result as MietteResult;
use pretty_env_logger::env_logger::{Env, Logger};
use pretty_env_logger::{env_logger, formatted_builder};
use rayon::prelude::*;
use std::io::prelude::*;
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
    let mut command = Command::new("python")
        .arg(script)
        .stdin(Stdio::piped())
        .spawn()
        .with_context(|| ScriptError::CommandError(script.to_owned()))?;

    {
        let stdin = command.stdin.as_mut().ok_or_else(|| {
            ScriptError::CommandError(format!("Failed to open stdin for {}", script))
        })?;
        let mut f = std::fs::File::open(input_file)
            .with_context(|| ScriptError::FileOpenError(input_file.to_owned()))?;
        let mut buf = String::new().into_bytes();
        f.read_to_end(&mut buf)
            .with_context(|| ScriptError::FileReadError(input_file.to_owned()))?;
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
fn main() -> MietteResult<()> {
    // Set up the logging and tracing infrastructure
    let env: Logger = formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_default())
        .build();
    let env: Env = Env::new().filter("MY_LOG").write_style("MY_LOG_STYLE");
    env_logger::init_from_env(env);

    // Define the file name
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
        .with_context(|| ScriptError::CommandError("Failed to execute scripts".to_owned()))
        .unwrap();

    info!("Execution completed successfully");
    Ok(())
}
