use crate::CliDiagnostic;
use crate::diagnostics::StdinDiagnostic;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::runner::process_file::{
    DiffKind, FileStatus, Message, ProcessFile, ProcessStdinFilePayload, WorkspaceFile,
    print_stdin_diagnostics,
};
use biome_analyze::RuleCategoriesBuilder;
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{Diagnostic, DiagnosticExt, Error, PrintDiagnostic, Severity, category};
use biome_service::WorkspaceError;
use biome_service::workspace::{
    FeaturesBuilder, FeaturesSupported, FileContent, FileFeaturesResult, ProcessFileParams,
    SupportsFeatureParams,
};
use tracing::debug;

pub(crate) struct FormatProcessFile;

impl ProcessFile for FormatProcessFile {
    fn process_file<Ctx>(
        ctx: &Ctx,
        workspace_file: &mut WorkspaceFile,
        _features_supported: &FeaturesSupported,
        max_diagnostics: u32,
        diagnostic_level: Severity,
    ) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        let execution = ctx.execution();
        let result = ctx
            .workspace()
            .process_file(ProcessFileParams {
                project_key: ctx.project_key(),
                path: workspace_file.path.clone(),
                content: FileContent::FromServer,
                categories: RuleCategoriesBuilder::default().with_syntax().build(),
                only: vec![],
                skip: vec![],
                enabled_rules: vec![],
                fix_file_mode: None,
                suppression_reason: None,
                format: true,
                write: execution.requires_write_access(),
                include_code_fix: false,
                max_diagnostics: Some(max_diagnostics),
                diagnostic_level,
                enforce_assist: false,
                skip_parse_errors: execution.should_skip_parse_errors(),
            })
            .with_file_path_and_code(workspace_file.path.to_string(), category!("format"))?;

        if result.parse_errors > 0 && execution.should_skip_parse_errors() {
            ctx.push_message(Message::from(
                SkippedDiagnostic.with_file_path(workspace_file.path.to_string()),
            ));
            return Ok(FileStatus::Ignored);
        }

        let output = result.output;
        let diagnostics: Vec<_> = result
            .diagnostics
            .into_iter()
            // Formatting is usually blocked by errors, so we want to print only diagnostics that
            // Have error severity
            .filter_map(|diagnostic| {
                if diagnostic.severity() >= Severity::Error {
                    Some(Error::from(diagnostic))
                } else {
                    None
                }
            })
            .collect();
        let content = if diagnostics.is_empty() {
            String::new()
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

        if result.format_with_errors_disabled {
            return Err(Message::from(
                WorkspaceError::format_with_errors_disabled()
                    .with_file_path(workspace_file.path.to_string()),
            ));
        }

        debug!(
            "Format output is different from input: {}",
            output.is_some()
        );
        if let Some(output) = output {
            if execution.requires_write_access() {
                Ok(FileStatus::Changed(workspace_file.write_to_disk(output)?))
            } else {
                Ok(FileStatus::Message(Message::Diff {
                    file_name: workspace_file.path.to_string(),
                    old: workspace_file.input()?,
                    new: output,
                    diff_kind: DiffKind::Format,
                }))
            }
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
            features: FeaturesBuilder::new().with_formatter().build(),
            inline_config: None,
            skip_ignore_check,
            not_requested_features: FeaturesBuilder::new()
                .with_all()
                .without_formatter()
                .build(),
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
        if file_features.supports_format() {
            let result = workspace.process_file(ProcessFileParams {
                project_key,
                path: biome_path.clone(),
                content: FileContent::from_client(content),
                categories: RuleCategoriesBuilder::default().with_syntax().build(),
                only: vec![],
                skip: vec![],
                enabled_rules: vec![],
                fix_file_mode: None,
                suppression_reason: None,
                format: true,
                write: true,
                include_code_fix: false,
                max_diagnostics: Some(execution.get_max_diagnostics(cli_options)),
                diagnostic_level: cli_options.diagnostic_level,
                enforce_assist: false,
                skip_parse_errors: execution.should_skip_parse_errors(),
            })?;
            let source = result.output.as_deref().unwrap_or(content);
            print_stdin_diagnostics(console, cli_options, biome_path, source, result.diagnostics);
            if result.format_with_errors_disabled {
                return Err(WorkspaceError::format_with_errors_disabled().into());
            }
            console.append(markup! {{source}});
            Ok(())
        } else {
            console.append(markup! {
                {content}
            });
            console.error(markup! {
                <Warn>"The content was not formatted because the formatter is currently disabled."</Warn>
            });
            Err(StdinDiagnostic::new_not_formatted().into())
        }
    }
}
