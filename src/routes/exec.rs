//! Handles server-side command execution
//!
//! # Warning
//! You must be *really* careful when working on this code, since it is a great point for server-side code injection.

use std::process::{Command, Stdio};

use rocket::response::stream::ReaderStream;

/// Command executor router
#[get("/exec/<command>/<host>")]
pub fn exec_command(
    command: &str,
    host: &str,
) -> Result<ReaderStream![tokio::process::ChildStdout], String> {
    info!("Got request to execute command: {}", command);
    
    // Parse the command into something we can run
    let command = match command {
        "ping" => format!("ping -c 4 -w15 {}", host),
        "ping6" => format!("ping -6 -c 4 -w15 {}", host),
        "traceroute" => format!("traceroute -4 -w2 {}", host),
        "traceroute6" => format!("traceroute -6 -w2 {}", host),
        _ => return Err(format!("Command not allowed: {}", command)),
    };
    info!("Command has been translated to: {}", command);

    // Start a subprocess with the command and keep its stdout stream
    let child = Command::new("sh")
        .arg("-c")
        .arg(format!("echo \"$ {}\" && {}", command, command))
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // Return the output of the command as a stream
    // let output = child.stdout.take().unwrap();
    let output = tokio::process::ChildStdout::from_std(child.stdout.unwrap()).unwrap();
    Ok(ReaderStream::one(output))
}
