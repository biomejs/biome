use super::{determine_fix_file_mode, FixFileModeOptions};
use crate::cli_options::CliOptions;
use crate::commands::{get_files_to_process_with_cli_options, CommandRunner};
use crate::{CliDiagnostic, Execution, TraversalMode};
use biome_configuration::analyzer::RuleSelector;
use biome_configuration::css::PartialCssLinter;
use biome_configuration::javascript::PartialJavascriptLinter;
use biome_configuration::json::PartialJsonLinter;
use biome_configuration::vcs::PartialVcsConfiguration;
use biome_configuration::{
    PartialConfiguration, PartialFilesConfiguration, PartialGraphqlLinter,
    PartialLinterConfiguration,
};
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::FileSystem;
use biome_service::configuration::LoadedConfiguration;
use biome_service::{DynRef, Workspace, WorkspaceError};
use std::ffi::OsString;

pub(crate) struct LintCommandPayload {
    pub(crate) apply: bool,
    pub(crate) apply_unsafe: bool,
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) unsafe_: bool,
    pub(crate) suppress: bool,
    pub(crate) linter_configuration: Option<PartialLinterConfiguration>,
    pub(crate) vcs_configuration: Option<PartialVcsConfiguration>,
    pub(crate) files_configuration: Option<PartialFilesConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
    pub(crate) javascript_linter: Option<PartialJavascriptLinter>,
    pub(crate) json_linter: Option<PartialJsonLinter>,
    pub(crate) css_linter: Option<PartialCssLinter>,
    pub(crate) graphql_linter: Option<PartialGraphqlLinter>,
}

impl CommandRunner for LintCommandPayload {
    const COMMAND_NAME: &'static str = "lint";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        _fs: &DynRef<'_, dyn FileSystem>,
        _console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError> {
        let LoadedConfiguration {
            configuration: mut fs_configuration,
            ..
        } = loaded_configuration;

        fs_configuration.merge_with(PartialConfiguration {
            linter: if fs_configuration
                .linter
                .as_ref()
                .is_some_and(PartialLinterConfiguration::is_disabled)
            {
                None
            } else {
                if let Some(linter) = self.linter_configuration.as_mut() {
                    // Don't overwrite rules from the CLI configuration.
                    linter.rules = None;
                }
                self.linter_configuration.clone()
            },
            files: self.files_configuration.clone(),
            vcs: self.vcs_configuration.clone(),
            ..Default::default()
        });

        if self.css_linter.is_some() {
            let css = fs_configuration.css.get_or_insert_with(Default::default);
            css.linter.merge_with(self.css_linter.clone());
        }

        if self.graphql_linter.is_some() {
            let graphql = fs_configuration
                .graphql
                .get_or_insert_with(Default::default);
            graphql.linter.merge_with(self.graphql_linter.clone());
        }
        if self.javascript_linter.is_some() {
            let javascript = fs_configuration
                .javascript
                .get_or_insert_with(Default::default);
            javascript.linter.merge_with(self.javascript_linter.clone());
        }
        if self.json_linter.is_some() {
            let json = fs_configuration.json.get_or_insert_with(Default::default);
            json.linter.merge_with(self.json_linter.clone());
        }

        Ok(fs_configuration)
    }

    fn get_files_to_process(
        &self,
        fs: &DynRef<'_, dyn FileSystem>,
        configuration: &PartialConfiguration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        let paths = get_files_to_process_with_cli_options(
            self.since.as_deref(),
            self.changed,
            self.staged,
            fs,
            configuration,
        )?
        .unwrap_or(self.paths.clone());

        Ok(paths)
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        self.stdin_file_path.as_deref()
    }

    fn should_write(&self) -> bool {
        self.write || self.fix
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        _workspace: &dyn Workspace,
    ) -> Result<Execution, CliDiagnostic> {
        let fix_file_mode = determine_fix_file_mode(
            FixFileModeOptions {
                apply: self.apply,
                apply_unsafe: self.apply_unsafe,
                write: self.write,
                fix: self.fix,
                unsafe_: self.unsafe_,
                suppress: self.suppress,
            },
            console,
        )?;
        Ok(Execution::new(TraversalMode::Lint {
            fix_file_mode,
            stdin: self.get_stdin(console)?,
            only: self.only.clone(),
            skip: self.skip.clone(),
            vcs_targeted: (self.staged, self.changed).into(),
            suppress: self.suppress,
        })
        .set_report(cli_options))
    }
}
