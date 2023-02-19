use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    // Define the file names
    let text_file1 = "input.txt";
    let text_file2 = "input.txt";

    // Define the command to run script1.rs with text_file1 piped in
    let mut script1_command = Command::new("python")
        .arg("entity.py")
        .stdin(Stdio::piped())
        .spawn()?;
    {
        let stdin = script1_command.stdin.as_mut().unwrap();
        let mut f = std::fs::File::open(text_file1)?;
        let mut buf = String::new().into_bytes();
        f.read_to_end(&mut buf)?;
        stdin.write_all(&buf)?;
    }
    let result1 = script1_command.wait()?;

    // Define the command to run script2.rs with text_file2 piped in
    let mut script2_command = Command::new("python")
        .arg("triagram.py")
        .stdin(Stdio::piped())
        .spawn()?;
    {
        let stdin = script2_command.stdin.as_mut().unwrap();
        let mut f = std::fs::File::open(text_file2)?;
        let mut buf = String::new().into_bytes();
        f.read_to_end(&mut buf)?;
        stdin.write_all(&buf)?;
    }
    let result2 = script2_command.wait()?;

    // Print the output of the two scripts
    println!("{:?}", result1);
    println!("{:?}", result2);

    Ok(())
}
