use crate::CliDiagnostic;
use biome_fs::FileSystem;
use biome_service::{DynRef, PartialConfiguration};
use std::ffi::OsString;

pub(crate) fn get_changed_files(
    fs: &DynRef<'_, dyn FileSystem>,
    configuration: &PartialConfiguration,
    since: Option<String>,
) -> Result<Vec<OsString>, CliDiagnostic> {
    let default_branch = configuration
        .vcs
        .as_ref()
        .and_then(|v| v.default_branch.as_ref());

    let base = match (since.as_ref(), default_branch) {
        (Some(since), Some(_)) => since,
        (Some(since), None) => since,
        (None, Some(branch)) => branch,
        (None, None) => return Err(CliDiagnostic::incompatible_end_configuration("The `--changed` flag was set, but Biome couldn't determine the base to compare against. Either set configuration.vcs.defaultBranch or use the --since argument.")),
    };

    let changed_files = fs.get_changed_files(base)?;

    let filtered_changed_files = changed_files.iter().map(OsString::from).collect::<Vec<_>>();

    if filtered_changed_files.is_empty() {
        return Err(CliDiagnostic::no_files_processed());
    }

    Ok(filtered_changed_files)
}
