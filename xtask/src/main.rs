use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context, Result};
use clap::{arg, command, value_parser, ArgAction};
use log::info;
use pretty_env_logger::env_logger;
use rayon::prelude::*;
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

#[instrument]
fn main() -> Result<()> {
    let start_time = std::time::Instant::now();

    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    if let Err(e) = try_main() {
        eprintln!("{}", anyhow::anyhow!(e.to_string()));
        std::process::exit(-1);
    }

    info!(
        "Execution completed successfully in {elapsed_time:?}",
        elapsed_time = start_time.elapsed()
    );

    Ok(())
}

fn try_main() -> Result<()> {
    let matches = command!() // Requires `cargo` feature.
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file")
                // We don't have syntax yet for optional options, so manually calling `required`
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug ... "Turn debugging information on"))
        .subcommand(
            clap::Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .subcommand(
            clap::Command::new("run").about("does executing python things").arg(
                arg!(-f --file <FILE> "Runs custom file")
                    .required(false)
                    .value_parser(value_parser!(PathBuf)),
            ),
        )
        .get_matches();

    run_xtask(matches)?;

    Ok(())
}
// In the main() function, instead of calling anyhow::anyhow!(e.to_string()), you can simply use
// e.to_string().into(), which will convert the error into an anyhow::Error type.
//
// The if let Some(matches) = matches.subcommand_matches("run") block can be refactored to avoid
// repeating code. You can create a separate function for running a single script file and call it
// within the if block. This would make the code more modular and easier to read.
//
// In the run() function, you may want to consider using the try_par_iter() function from the rayon
// crate instead of par_bridge(). This function will automatically collect any errors that occur
// during parallel execution and return them as a single Result object.
//
// In the execute_script_batch() function, you can use the with_extension() method from the
// std::path::Path module to add the .txt extension to the output file path. This would make the
// code more concise and easier to read.
fn run_xtask(matches: clap::ArgMatches) -> Result<(), anyhow::Error> {
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {}", name);
    }
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }
    match matches.get_one::<u8>("debug").expect("Count's are defaulted") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Debug mode? Don't be crazy"),
    }
    if let Some(matches) = matches.subcommand_matches("test") {
        // '$ cargo xtask test' was run.
        if matches.get_flag("list") {
            // '$ cargo xtask test -l' was run.
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
    if let Some(matches) = matches.subcommand_matches("run") {
        // '$ cargo xtask run' was run.
        if let Some(file) = matches.get_one::<PathBuf>("file") {
            // '$ cargo xtask run -f' was run.
            println!("Running file: `{file}`", file = file.display());
            //TODO: run custom files only.
        } else {
            // Only '$ cargo xtask run' was run without flags.
            println!("`[run]`: Processing all default files...");
            run_python()?;
        }
    };
    Ok(())
}

fn run_python() -> Result<()> {
    let input_files_batch = vec!["in/mlk.txt", "in/alice.txt"];
    let scripts_batch = vec![
        ("triagram.py", "out/triagram/"),
        // ("tensor.py", "out/tensor/"),
        ("entity.py", "out/entity/"),
    ];

    scripts_batch.into_iter().par_bridge().for_each(|(script, out_dir)| {
        input_files_batch
            .iter()
            .par_bridge()
            .map(|input_file| (input_file, script, out_dir)) // Create tuples
            .for_each(|(input_file, script, out_dir)| {
                trace_info_span_enter(script);
                execute_script_batch(script, input_file, out_dir).unwrap();
            });
    });

    Ok(())
}

/// Constructs a span at the info level. Enters this span, returning a guard that will exit the span
/// when dropped.
fn trace_info_span_enter(script: &str) {
    let span = info_span!("script: `{script}`", script = script);
    let _guard = span.enter();
}

// To solve this, you can try adding the project directory to the beginning of the
// input and output file paths, like this:
//
// * let output_file = format!("{}/{}{}.txt", out_dir, input_file_name, out_suffix);
// * let input_file = format!("{}/{}", project_dir, input_file);
// * let output_file = format!("{}/{}", out_dir, output_file);
fn execute_script_batch(script: &str, input_file: &str, out_dir: &str) -> Result<()> {
    let input_file_name = Path::new(input_file).file_stem().unwrap().to_str().unwrap();
    let output_file = format!("{}{}.txt", out_dir, input_file_name);

    fs::create_dir_all(out_dir).expect("Could not create output directory");
    let out_file = File::create(&output_file)
        .with_context(|| ScriptError::FileOpenError(output_file.to_owned()))
        .unwrap();

    let mut command = Command::new("python3")
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(out_file)
        .spawn()
        .with_context(|| ScriptError::CommandError(script.to_owned()))?;
    {
        let stdin = command.stdin.as_mut().ok_or_else(|| {
            ScriptError::CommandError(format!("Failed to open stdin for {}", script))
        })?;
        let buf = fs::read(input_file)
            .with_context(|| ScriptError::FileOpenError(input_file.to_owned()))?;
        stdin.write_all(&buf).with_context(|| ScriptError::WriteError(script.to_owned()))?;
    }
    command.wait().with_context(|| ScriptError::WaitError(script.to_owned()))?;

    Ok(())
}
