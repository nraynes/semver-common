use r_log::Logger;
use std::{ffi::OsStr, process::Command};

use crate::Alert;

pub fn run_command<I, S>(command: &str, args: I, logger: Option<&Logger>) -> Result<String, Alert>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(command).args(args).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    let status = output.status.code().ok_or(&stderr)?;
    if status == 0 {
        if let Some(l) = logger {
            l.debug(&stdout);
        }
        return Ok(stdout);
    }
    if let Some(l) = logger {
        l.debug(&stderr);
    }
    Err(Alert::from(stderr))
}
