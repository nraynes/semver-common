use r_log::Logger;
use std::{ffi::OsStr, fmt::Display, process::Command};

use crate::Alert;

pub fn run_command<I, S>(command: &str, args: I, logger: Option<&Logger>) -> Result<String, Alert>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr> + Display,
{
    let mut log_message = command.to_string();
    let cpargs: Vec<S> = args.into_iter().collect();
    for arg in &cpargs {
        log_message = format!("{} {}", log_message, arg);
    }
    let output = Command::new(command).args(cpargs).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    let status = output.status.code().ok_or(&stderr)?;

    if status == 0 {
        if let Some(l) = logger {
            l.debug(&format!("{}\n {}", log_message, stdout));
        }
        return Ok(stdout);
    }
    if let Some(l) = logger {
        l.debug(&format!("{}\n {}", log_message, stderr));
    }
    Err(Alert::from(stderr))
}
