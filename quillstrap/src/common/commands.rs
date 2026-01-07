#[allow(unused_imports)]
use crate::prelude::*;
use std::io;
use std::process::{Command, Stdio};

pub fn run_command(command: &str, show_output: bool) -> io::Result<()> {
    if show_output {
        info!("Executing \"{}\" in shell", command);
    }

    let mut parts = command.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let mut child = Command::new(program)
            .args(&args)
            .stdout(if !show_output {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .stderr(if !show_output {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .spawn()?;

        child.wait()?;
    }
    Ok(())
}

pub fn run_shell_command(command: &str, show_output: bool) -> io::Result<()> {
    if show_output {
        info!("Executing \"{}\" in shell", command);
    }
    
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(if !show_output {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .stderr(if !show_output {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .spawn()?;

    child.wait()?;
    Ok(())
}

pub fn run_shell_command_and_check_success(command: &str, show_output: bool) -> io::Result<bool> {
    if show_output {
        info!("Executing \"{}\" in shell", command);
    }

    let status = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(if !show_output {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .stderr(if !show_output {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .status()?;

    Ok(status.success())
}

pub fn run_command_get_output(command: &str) -> String {
    let mut parts = command.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let output = Command::new(program)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        return format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    String::new()
}

pub fn run_shell_command_get_output(command: &str) -> String {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute shell command");

    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

pub fn get_cores() -> u16 {
    run_command_get_output("nproc").parse::<u16>().unwrap_or(16)
}
