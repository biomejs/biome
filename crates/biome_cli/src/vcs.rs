use biome_fs::FileSystem;
use biome_service::{Configuration, DynRef, WorkspaceError};
use std::path::PathBuf;

/// This function checks if the VCS integration is enabled, and if so, it will attempts to resolve the
/// VCS root directory and the `.gitignore` file.
///
/// ## Returns
///
/// A tuple with VCS root folder and the contents of the `.gitignore` file
pub(crate) fn retrieve_gitignore_matches(
    file_system: &DynRef<'_, dyn FileSystem>,
    configuration: &Configuration,
    vcs_base_path: Option<PathBuf>,
) -> Result<(Option<PathBuf>, Vec<String>), WorkspaceError> {
    let Some(vcs) = &configuration.vcs else {
        return Ok((None, vec![]));
    };
    if vcs.is_enabled() {
        let vcs_base_path = match (vcs_base_path, &vcs.root) {
            (Some(vcs_base_path), Some(root)) => vcs_base_path.join(root),
            (None, Some(root)) => PathBuf::from(root),
            (Some(vcs_base_path), None) => vcs_base_path,
            (None, None) => return Err(WorkspaceError::vcs_disabled()),
        };
        if let Some(client_kind) = &vcs.client_kind {
            if !vcs.ignore_file_disabled() {
                let result = file_system
                    .auto_search(vcs_base_path, client_kind.ignore_file(), false)
                    .map_err(WorkspaceError::from)?;

                if let Some(result) = result {
                    return Ok((
                        Some(result.directory_path),
                        result
                            .content
                            .lines()
                            .map(String::from)
                            .collect::<Vec<String>>(),
                    ));
                }
            }
        }
    }
    Ok((None, vec![]))
}
