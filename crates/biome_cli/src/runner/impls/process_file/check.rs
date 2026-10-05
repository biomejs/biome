use crate::CliDiagnostic;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::runner::execution::AnalyzerSelectors;
use crate::runner::impls::process_file::lint_and_assist::LintAssistProcessFile;
use crate::runner::process_file::{
    DiffKind, FileStatus, Message, ProcessFile, ProcessStdinFilePayload, WorkspaceFile,
};
use biome_analyze::RuleCategoriesBuilder;
use biome_diagnostics::{DiagnosticExt, Error, Severity};
use biome_service::workspace::{FeaturesSupported, FileContent, ProcessFileParams};
use tracing::info;

pub(crate) struct CheckProcessFile;

impl ProcessFile for CheckProcessFile {
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
        if features_supported.supports_assist() {
            categories = categories.with_assist();
        }
        let AnalyzerSelectors { only, skip } = execution.analyzer_selectors();
        let fix_file_mode = execution.as_fix_file_mode();
        let result = ctx
            .workspace()
            .process_file(ProcessFileParams {
                project_key: ctx.project_key(),
                path: workspace_file.path.clone(),
                content: FileContent::FromServer,
                categories: categories.build(),
                only,
                skip,
                enabled_rules: vec![],
                fix_file_mode,
                suppression_reason: None,
                format: features_supported.supports_format(),
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
            return Ok(FileStatus::Unchanged);
        }

        let output = result.output;
        if !result.diagnostics.is_empty() || result.skipped_diagnostics > 0 {
            let content = if result.diagnostics.is_empty() {
                String::new()
            } else if fix_file_mode.is_some()
                && let Some(output) = &output
            {
                output.clone()
            } else {
                workspace_file.input()?
            };
            ctx.push_message(Message::Diagnostics {
                file_path: workspace_file.path.to_string(),
                content,
                diagnostics: result.diagnostics.into_iter().map(Error::from).collect(),
                skipped_diagnostics: result.skipped_diagnostics as u32,
                errors: result.errors,
                warnings: result.warnings,
                infos: result.infos,
            });
        }

        if result.format_with_errors_disabled {
            ctx.push_message(Message::from(
                biome_service::WorkspaceError::format_with_errors_disabled()
                    .with_file_path(workspace_file.path.to_string()),
            ));
            if let Some(output) = output {
                if execution.requires_write_access() {
                    workspace_file.write_to_disk(output)?;
                } else {
                    ctx.push_message(Message::Diff {
                        file_name: workspace_file.path.to_string(),
                        old: workspace_file.input()?,
                        new: output,
                        diff_kind: DiffKind::Format,
                    });
                }
            }
            return Ok(FileStatus::Message(Message::Failure));
        }

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
        LintAssistProcessFile::process_std_in(payload)
    }
}
