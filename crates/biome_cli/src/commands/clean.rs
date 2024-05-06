use crate::commands::daemon::biome_log_dir;
use crate::{CliDiagnostic, CliSession};
use std::fs::{create_dir, remove_dir_all};

/// Runs the clean command
pub fn clean(_cli_session: CliSession) -> Result<(), CliDiagnostic> {
    let logs_dir = biome_log_dir();
    remove_dir_all(logs_dir.clone()).and_then(|_| create_dir(logs_dir))?;
    Ok(())
}
