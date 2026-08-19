use crate::commands::daemon::default_biome_log_path;
use crate::{CliDiagnostic, CliSession};
use biome_console::{ConsoleExt, markup};
use biome_flags::biome_env;
use camino::Utf8PathBuf;
use std::fs::{create_dir, remove_dir_all};

/// Runs the clean command
pub fn clean(cli_session: CliSession) -> Result<(), CliDiagnostic> {
    let logs_path = biome_env()
        .value_for("BIOME_LOG_PATH")
        .map_or(default_biome_log_path(), Utf8PathBuf::from);
    remove_dir_all(logs_path.clone()).and_then(|_| create_dir(logs_path.clone()))?;
    let console = cli_session.app.console;
    console.log(markup! {
      <Info>"Successfully cleaned the folder "{&logs_path.to_string()}</Info>
    });
    Ok(())
}
