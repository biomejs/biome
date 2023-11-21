use biome_deserialize::StringSet;
use biome_fs::RomePath;
use biome_service::{
    configuration::FilesConfiguration,
    workspace::{FeatureName, IsPathIgnoredParams},
    Configuration,
};

use crate::{CliDiagnostic, CliSession};

pub(crate) fn store_changed_files(
    session: &mut CliSession,
    configuration: &mut Configuration,
    since: Option<String>,
) -> Result<(), CliDiagnostic> {
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

    let fs = &mut session.app.fs;
    let ws = &mut session.app.workspace;

    let changed_files = fs.get_changed_files(base)?;

    println!("changed files: {:?}", changed_files);

    let filtered_changed_files: Vec<String> = changed_files
        .iter()
        .filter(|file| {
            !ws.is_path_ignored(IsPathIgnoredParams {
                rome_path: RomePath::new(file),
                feature: FeatureName::Lint,
            })
            .unwrap_or(false)
        })
        .map(|file| file.to_string())
        .collect();

    println!("filtered changed files: {:?}", filtered_changed_files);

    if filtered_changed_files.is_empty() {
        return Err(CliDiagnostic::no_files_processed());
    }

    let files_config = configuration
        .files
        .get_or_insert_with(FilesConfiguration::default);

    let included_files = &mut files_config.include.get_or_insert_with(StringSet::default);

    included_files.clear();
    included_files.extend(filtered_changed_files);

    Ok(())
}
