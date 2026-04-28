use crate::{Alert, run_command};
use r_log::Logger;

/// Stages all changes in git and commits those changes with a supplied message.
pub fn commit_all(message: &str, logger: &Logger) -> Result<(), Alert> {
    run_command("git", ["add", "."], Some(logger))?;
    run_command("git", ["commit", "-m", message], Some(logger))?;
    Ok(())
}
