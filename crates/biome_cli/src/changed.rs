use biome_deserialize::StringSet;
use biome_service::{configuration::FilesConfiguration, settings::to_matcher, Configuration};

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

    let changed_files = fs.get_changed_files(base)?;

    let files_config = configuration
        .files
        .get_or_insert_with(FilesConfiguration::default);

    let included_files = &mut files_config.include;
    let ignored_files = &files_config.ignore;

    let included_matcher = to_matcher(included_files.as_ref())?;
    let ignored_matcher = to_matcher(ignored_files.as_ref())?;

    let filtered_changed_files: Vec<String> = match (included_matcher, ignored_matcher) {
        (Some(included), Some(ignored)) => changed_files
            .iter()
            .filter(|file| included.matches(file) && !ignored.matches(file))
            .map(|path| path.to_string())
            .collect(),
        (Some(included), None) => changed_files
            .iter()
            .filter(|file| included.matches(file))
            .map(|path| path.to_string())
            .collect(),
        (None, Some(ignored)) => changed_files
            .iter()
            .filter(|file| !ignored.matches(file))
            .map(|path| path.to_string())
            .collect(),
        (None, None) => changed_files.iter().map(|path| path.to_string()).collect(),
    };

    if filtered_changed_files.is_empty() {
        return Err(CliDiagnostic::no_files_processed());
    };

    let included_files = included_files.get_or_insert_with(StringSet::default);

    included_files.clear();
    included_files.extend(filtered_changed_files);

    Ok(())
}
