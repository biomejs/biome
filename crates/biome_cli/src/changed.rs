use std::process::{Command, Output};

use biome_deserialize::StringSet;
use biome_service::{configuration::FilesConfiguration, settings::to_matcher, Configuration};

use crate::CliDiagnostic;

pub(crate) fn store_changed_files(
    configuration: &mut Configuration,
    since: Option<String>,
) -> Result<(), CliDiagnostic> {
    let default_branch = configuration
        .vcs
        .as_ref()
        .map_or(None, |v| v.default_branch.as_ref());

    let base = match (since.as_ref(), default_branch) {
        (Some(since), Some(_)) => since,
        (Some(since), None) => since,
        (None, Some(branch)) => branch,
        (None, None) => return Err(CliDiagnostic::incompatible_end_configuration("The `--changed` flag was set, but Biome couldn't determine the base to compare against. Either set configuration.vcs.defaultBranch or use the --since argument.")),
    };

    let output = run_git_diff(base)?;

    if !output.status.success() {
        todo!("Handle error when git command fails");
    }

    let files_config = configuration
        .files
        .get_or_insert_with(FilesConfiguration::default);

    let included_files = files_config.include.get_or_insert_with(StringSet::default);

    let matcher = to_matcher(Some(&included_files))?;

    // Process the output to get 'changed_files'
    let changed_files: Vec<String> = match matcher {
        Some(matcher) if !included_files.is_empty() => {
            // Filter and collect lines that match and are non-empty
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| !line.is_empty() && matcher.matches(line))
                .map(ToString::to_string)
                .collect()
        }
        _ => {
            // Collect all non-empty lines if no matcher or 'included_files' is empty
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| !line.is_empty())
                .map(ToString::to_string)
                .collect()
        }
    };

    if changed_files.len() == 0 {
        return Err(CliDiagnostic::no_files_processed());
    };

    included_files.clear();
    included_files.extend(changed_files);

    Ok(())
}

fn run_git_diff(base: &str) -> std::io::Result<Output> {
    return Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(format!("{}...HEAD", base))
        .output();
}
