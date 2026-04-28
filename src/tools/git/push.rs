use crate::{Alert, run_command};
use r_log::Logger;

/// Pushes any commited changes in git to the authenticated remote repository.
pub fn push(logger: &Logger) -> Result<(), Alert> {
    run_command("git", ["push"], Some(logger))?;
    Ok(())
}
