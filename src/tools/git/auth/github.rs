use r_log::Logger;
use std::{collections::HashMap, fs::File, io::Write};

use crate::{Alert, run_command};

/// Caches credentials in git for authenticating with Github as remote origin.
pub fn set_remote(env: &HashMap<String, String>, logger: &Logger) -> Result<(), Alert> {
    logger.info("Authenticating with Github");
    let actor = env
        .get("GITHUB_ACTOR")
        .ok_or("GITHUB_ACTOR not in environment variables.")?;
    let token = env
        .get("GITHUB_TOKEN")
        .ok_or("GITHUB_TOKEN not in environment variables.")?;
    run_command(
        "git",
        ["config", "--global", "credential.helper", "store"],
        Some(logger),
    )?;
    println!("Creating file...");
    let mut github_credentials = File::create("~/.git-credentials")?;
    println!("Writing file...");
    github_credentials.write_all(format!("https://${}:${}@github.com", actor, token).as_bytes())?;
    Ok(())
}
