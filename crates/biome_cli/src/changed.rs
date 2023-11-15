use std::process::Command;

use biome_console::{markup, LogLevel};
use biome_deserialize::StringSet;
use biome_service::{configuration::FilesConfiguration, Configuration};

use crate::{cli_options::CliOptions, CliDiagnostic, CliSession};

pub(crate) fn store_changed_files(
    session: &mut CliSession,
    configuration: &mut Configuration,
    cli_options: &CliOptions,
) -> Result<(), CliDiagnostic> {
    let default_branch = configuration
        .vcs
        .as_ref()
        .map_or(None, |v| v.default_branch.as_ref());

    let comitish = match (cli_options.since.as_ref(), default_branch) {
        (Some(since), Some(_)) => since,
        (Some(since), None) => since,
        (None, Some(branch)) => branch,
        (None, None) => return Err(CliDiagnostic::incompatible_end_configuration("The `--changed` flag was set, but Biome couldn't determine the base to compare against. Either set configuration.vcs.defaultBranch or use the --since argument.")),
    };

    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(format!("{}...HEAD", comitish))
        .output()?;

    if !output.status.success() {
        todo!("Handle error when git command fails");
    }

    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    let files_config = configuration
        .files
        .get_or_insert_with(FilesConfiguration::default);

    let included_files = files_config.include.get_or_insert_with(StringSet::default);
    println!("files {:?}", &files);

    included_files.clear();
    included_files.extend(files);

    println!("included files {:?}", included_files);

    Ok(())
}
