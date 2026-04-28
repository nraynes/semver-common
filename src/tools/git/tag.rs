use crate::{Alert, run_command};
use r_log::Logger;

/// Creates a new git tag with a supplied name and message.
pub fn tag(name: &str, message: &str, logger: &Logger) -> Result<(), Alert> {
    run_command(
        "git",
        ["tag", "-a", name, "-m", &format!("{} {}", message, name)],
        Some(logger),
    )?;
    run_command("git", ["push", "--tags"], Some(logger))?;
    Ok(())
}
