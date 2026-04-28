use crate::{Alert, run_command};
use r_log::Logger;

/// Fetch from the remote repository to ensure commit history is updated.
pub fn fetch(logger: &Logger) -> Result<(), Alert> {
    run_command("git", ["fetch", "--tags", "--prune"], Some(logger))?;
    Ok(())
}
