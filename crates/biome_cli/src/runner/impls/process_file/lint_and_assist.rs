use crate::CliDiagnostic;
use crate::diagnostics::StdinDiagnostic;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::runner::execution::AnalyzerSelectors;
use crate::runner::process_file::{
    FileStatus, Message, ProcessFile, ProcessStdinFilePayload, WorkspaceFile,
};
use biome_analyze::RuleCategoriesBuilder;
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{Diagnostic, DiagnosticExt, Error, PrintDiagnostic, Severity};
use biome_service::WorkspaceError;
use biome_service::workspace::{
    FeaturesSupported, FileContent, FileFeaturesResult, ProcessFileParams, SupportsFeatureParams,
};
use tracing::info;

pub struct LintAssistProcessFile;

impl ProcessFile for LintAssistProcessFile {
    fn process_file<Ctx>(
        ctx: &Ctx,
        workspace_file: &mut WorkspaceFile,
        features_supported: &FeaturesSupported,
        max_diagnostics: u32,
        diagnostic_level: Severity,
    ) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        let execution = ctx.execution();
        let mut categories = RuleCategoriesBuilder::default().with_syntax();
        if features_supported.supports_lint() {
            categories = categories.with_lint();
        }
        if features_supported.supports_assist() && (execution.is_check() || execution.is_ci()) {
            categories = categories.with_assist();
        }
        let categories = categories.build();
        let AnalyzerSelectors { only, skip } = execution.analyzer_selectors();
        let fix_file_mode = execution.as_fix_file_mode();
        let suppression_reason = fix_file_mode.map(|_| {
            if execution.suppress() && execution.suppression_reason().is_none() {
                String::from("ignored using `--suppress`")
            } else {
                execution
                    .suppression_reason()
                    .unwrap_or("<explanation>")
                    .to_string()
            }
        });
        let result = ctx
            .workspace()
            .process_file(ProcessFileParams {
                project_key: ctx.project_key(),
                path: workspace_file.path.clone(),
                content: FileContent::FromServer,
                categories,
                only,
                skip,
                enabled_rules: vec![],
                fix_file_mode,
                suppression_reason,
                format: false,
                write: execution.requires_write_access(),
                include_code_fix: true,
                max_diagnostics: Some(max_diagnostics),
                diagnostic_level,
                enforce_assist: execution.should_enforce_assist(),
                skip_parse_errors: execution.should_skip_parse_errors(),
            })
            .with_file_path_and_code(
                workspace_file.path.to_string(),
                execution.as_diagnostic_category(),
            )?;

        info!(
            "Process file summary result. Errors {}, skipped fixes {}, applied fixes {}",
            result.errors, result.skipped_suggested_fixes, result.applied_fixes
        );

        if fix_file_mode.is_some() {
            ctx.push_message(Message::SkippedFixes {
                skipped_suggested_fixes: result.skipped_suggested_fixes,
            });
        }

        if result.parse_errors > 0 && execution.should_skip_parse_errors() {
            ctx.push_message(Message::from(
                SkippedDiagnostic.with_file_path(workspace_file.path.to_string()),
            ));
            return Ok(FileStatus::Ignored);
        }

        let output = result.output;
        let no_diagnostics = result.diagnostics.is_empty() && result.skipped_diagnostics == 0;

        if !no_diagnostics {
            let diagnostics: Vec<Error> = result.diagnostics.into_iter().map(Error::from).collect();
            let content = if diagnostics.is_empty() {
                String::new()
            } else if let Some(output) = &output {
                output.clone()
            } else {
                workspace_file.input()?
            };

            ctx.push_message(Message::Diagnostics {
                file_path: workspace_file.path.to_string(),
                content,
                diagnostics,
                skipped_diagnostics: result.skipped_diagnostics as u32,
                errors: result.errors,
                warnings: result.warnings,
                infos: result.infos,
            });
        }

        if let Some(output) = output {
            Ok(FileStatus::Changed(workspace_file.write_to_disk(output)?))
        } else {
            Ok(FileStatus::Unchanged)
        }
    }

    fn process_std_in(payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic> {
        let ProcessStdinFilePayload {
            workspace,
            content,
            project_key,
            biome_path,
            console,
            cli_options,
            execution,
            skip_ignore_check,
        } = payload;

        let FileFeaturesResult {
            features_supported: file_features,
        } = workspace.file_features(SupportsFeatureParams {
            project_key,
            path: biome_path.clone(),
            features: execution.wanted_features(),
            inline_config: None,
            skip_ignore_check,
            not_requested_features: execution.not_requested_features(),
        })?;

        if file_features.is_ignored() {
            console.append(markup! {{content}});
            return Ok(());
        }

        if file_features.is_protected() {
            let protected_diagnostic = WorkspaceError::protected_file(biome_path.to_string());
            if protected_diagnostic.tags().is_verbose() {
                if cli_options.verbose {
                    console.error(markup! {{PrintDiagnostic::verbose(&protected_diagnostic)}})
                }
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&protected_diagnostic)}})
            }
            console.append(markup! {{content}});

            return Ok(());
        };

        let AnalyzerSelectors { only, skip } = execution.analyzer_selectors();
        let mut categories = RuleCategoriesBuilder::default().with_syntax();
        if file_features.supports_lint() {
            categories = categories.with_lint();
        }
        if file_features.supports_assist() && (execution.is_check() || execution.is_ci()) {
            categories = categories.with_assist();
        }
        let fix_file_mode = execution.as_fix_file_mode();
        let suppression_reason = fix_file_mode.map(|_| {
            if execution.suppress() && execution.suppression_reason().is_none() {
                String::from("ignored using `--suppress`")
            } else {
                execution
                    .suppression_reason()
                    .unwrap_or("<explanation>")
                    .to_string()
            }
        });
        let result = workspace.process_file(ProcessFileParams {
            project_key,
            path: biome_path.clone(),
            content: FileContent::from_client(content),
            categories: categories.build(),
            only,
            skip,
            enabled_rules: vec![],
            fix_file_mode,
            suppression_reason,
            format: execution.is_check()
                && execution.requires_write_access()
                && file_features.supports_format(),
            write: execution.requires_write_access(),
            include_code_fix: true,
            max_diagnostics: Some(execution.get_max_diagnostics(cli_options)),
            diagnostic_level: cli_options.diagnostic_level,
            enforce_assist: execution.should_enforce_assist(),
            skip_parse_errors: execution.should_skip_parse_errors(),
        })?;
        let source = result.output.as_deref().unwrap_or(content);
        console.append(markup! {{source}});

        if result.output.is_none() && !execution.requires_write_access() {
            Err(StdinDiagnostic::new_not_formatted().into())
        } else {
            Ok(())
        }
    }
}
