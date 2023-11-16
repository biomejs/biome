use std::process::Command;

use biome_deserialize::StringSet;
use biome_service::{configuration::FilesConfiguration, settings::to_matcher, Configuration};

use crate::{cli_options::CliOptions, CliDiagnostic};

pub(crate) fn store_changed_files(
    configuration: &mut Configuration,
    cli_options: &CliOptions,
) -> Result<(), CliDiagnostic> {
    let default_branch = configuration
        .vcs
        .as_ref()
        .map_or(None, |v| v.default_branch.as_ref());

    let base = match (cli_options.since.as_ref(), default_branch) {
        (Some(since), Some(_)) => since,
        (Some(since), None) => since,
        (None, Some(branch)) => branch,
        (None, None) => return Err(CliDiagnostic::incompatible_end_configuration("The `--changed` flag was set, but Biome couldn't determine the base to compare against. Either set configuration.vcs.defaultBranch or use the --since argument.")),
    };

    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(format!("{}...HEAD", base))
        .output()?;

    if !output.status.success() {
        todo!("Handle error when git command fails");
    }

    let files_config = configuration
        .files
        .get_or_insert_with(FilesConfiguration::default);

    let included_files = files_config.include.get_or_insert_with(StringSet::default);

    let matcher = to_matcher(Some(&included_files))?;

    let changed_files: Vec<String> = if let Some(matcher) = matcher {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| !line.is_empty())
            .filter(|line| matcher.matches(line))
            .map(|line| line.to_string())
            .collect()
    } else {
        todo!("Figure out how to handle this error properly");
    };

    if changed_files.len() == 0 {
        return Err(CliDiagnostic::no_files_processed());
    };

    included_files.clear();
    included_files.extend(changed_files);

    Ok(())
}
