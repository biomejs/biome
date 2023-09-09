use crate::cli_options::CliOptions;
use crate::diagnostics::{DisabledVcs, NoVcsFolderFound};
use crate::{CliDiagnostic, CliSession};
use biome_diagnostics::PrintDiagnostic;
use rome_console::{markup, ConsoleExt};
use rome_deserialize::StringSet;
use rome_service::configuration::vcs::{VcsClientKind, VcsConfiguration};
use rome_service::configuration::FilesConfiguration;
use rome_service::{Configuration, WorkspaceError};
use std::path::PathBuf;

/// This function will check if the configuration is set to use the VCS integration and try to
/// read the ignored files.
pub(crate) fn store_path_to_ignore_from_vcs(
    session: &mut CliSession,
    configuration: &mut Configuration,
    vcs_base_path: Option<PathBuf>,
    cli_options: &CliOptions,
) -> Result<(), CliDiagnostic> {
    let Some(vcs) = &configuration.vcs else {
        return Ok(());
    };
    if vcs.is_enabled() {
        let vcs_base_path = match (vcs_base_path, &vcs.root) {
            (Some(vcs_base_path), Some(root)) => vcs_base_path.join(root),
            (None, Some(root)) => PathBuf::from(root),
            (Some(vcs_base_path), None) => vcs_base_path,
            (None, None) => {
                let console = &mut session.app.console;
                let diagnostic = DisabledVcs {};
                console.error(markup! {
					{if cli_options.verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
                });
                return Ok(());
            }
        };

        let files_to_ignore = read_vcs_ignore_file(session, vcs_base_path, vcs)?;

        if !files_to_ignore.is_empty() {
            let files = configuration
                .files
                .get_or_insert_with(FilesConfiguration::default);
            let ignored_files = files.ignore.get_or_insert_with(StringSet::default);
            ignored_files.extend(files_to_ignore);
        }
    }
    Ok(())
}

pub(crate) fn read_vcs_ignore_file(
    session: &mut CliSession,
    current_directory: PathBuf,
    configuration: &VcsConfiguration,
) -> Result<Vec<String>, CliDiagnostic> {
    if !configuration.is_enabled() {
        return Ok(vec![]);
    }
    let file_system = &session.app.fs;

    if let Some(client_kind) = &configuration.client_kind {
        match client_kind {
            VcsClientKind::Git => {
                let git_folder = current_directory.join(".git");

                if !file_system.path_exists(git_folder.as_path()) {
                    return Err(CliDiagnostic::NoVcsFolderFound(NoVcsFolderFound {
                        path: git_folder.display().to_string(),
                        source: None,
                    }));
                }
            }
        }
        if !configuration.ignore_file_disabled() {
            let result = file_system
                .auto_search(current_directory, client_kind.ignore_file(), false)
                .map_err(WorkspaceError::from)?;

            if let Some(result) = result {
                return Ok(result
                    .content
                    .lines()
                    // remove empty lines
                    .filter(|line| !line.is_empty())
                    .filter_map(|item| {
                        let line = item.to_string();
                        // remove comments
                        if !line.starts_with('#') {
                            Some(line)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>());
            }
        }
    }

    Ok(vec![])
}
