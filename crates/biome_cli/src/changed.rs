use crate::CliDiagnostic;
use biome_configuration::Configuration;
use biome_fs::FileSystem;
use camino::Utf8Path;
use std::ffi::OsString;

/// Returns the list of changed file paths that exist on disk, computed relative to a base reference.
///
/// The base reference is taken from `since` when present, otherwise from `configuration.vcs.default_branch`.
/// If neither is available, the function returns an error. Paths reported by the VCS are filtered to
/// include only non-empty entries that either exist as given or exist when resolved relative to the
/// repository working directory.
///
/// # Returns
///
/// A `Vec<OsString>` containing the changed paths that exist, or a `CliDiagnostic` error if base
/// determination fails or an underlying filesystem operation fails.
///
/// # Examples
///
/// ```
/// // Assuming `fs: &dyn FileSystem` and `configuration: &Configuration` are available:
/// // let changed = get_changed_files(fs, configuration, Some("main"))?;
/// # let _ = ();
/// ```
pub(crate) fn get_changed_files(
    fs: &dyn FileSystem,
    configuration: &Configuration,
    since: Option<&str>,
) -> Result<Vec<OsString>, CliDiagnostic> {
    let default_branch = configuration
        .vcs
        .as_ref()
        .and_then(|v| v.default_branch.as_ref());

    let base = match (since, default_branch) {
        (Some(since), Some(_)) => since,
        (Some(since), None) => since,
        (None, Some(branch)) => branch,
        (None, None) => {
            return Err(CliDiagnostic::incompatible_end_configuration(
                "The `--changed` flag was set, but Biome couldn't determine the base to compare against. Either set configuration.vcs.defaultBranch or use the --since argument.",
            ));
        }
    };

    let changed_files = fs.get_changed_files(base)?;
    let working_directory = fs.working_directory();

    let filtered_changed_files = changed_files
        .into_iter()
        .filter_map(|path| {
            if path.is_empty() {
                return None;
            }

            let candidate = Utf8Path::new(path.as_str());

            let exists = fs.path_exists(candidate)
                || (!candidate.is_absolute()
                    && working_directory.as_ref().is_some_and(|dir| {
                        let absolute = dir.join(candidate);
                        fs.path_exists(absolute.as_path())
                    }));

            exists.then(|| OsString::from(path))
        })
        .collect::<Vec<_>>();

    Ok(filtered_changed_files)
}

pub(crate) fn get_staged_files(fs: &dyn FileSystem) -> Result<Vec<OsString>, CliDiagnostic> {
    let staged_files = fs.get_staged_files()?;

    let filtered_staged_files = staged_files.iter().map(OsString::from).collect::<Vec<_>>();

    Ok(filtered_staged_files)
}
