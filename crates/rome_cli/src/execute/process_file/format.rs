use crate::execute::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{
    DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions,
};
use crate::execute::TraversalMode;
use crate::FormatterReportFileDetail;
use biome_diagnostics::{category, DiagnosticExt};
use rome_service::workspace::RuleCategories;
use std::path::Path;
use std::sync::atomic::Ordering;

pub(crate) fn format<'ctx>(ctx: &'ctx SharedTraversalOptions<'ctx, '_>, path: &Path) -> FileResult {
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    format_with_guard(ctx, &mut workspace_file)
}

pub(crate) fn format_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
    let diagnostics_result = workspace_file
        .guard()
        .pull_diagnostics(RuleCategories::SYNTAX, max_diagnostics.into())
        .with_file_path_and_code(
            workspace_file.path.display().to_string(),
            category!("format"),
        )?;

    let input = workspace_file.input()?;
    let (should_write, ignore_errors) = match ctx.execution.traversal_mode {
        TraversalMode::Format {
            write,
            ignore_errors,
            ..
        } => (write, ignore_errors),

        _ => (
            ctx.execution.is_check_apply() || ctx.execution.is_check_apply_unsafe(),
            false,
        ),
    };

    if diagnostics_result.errors > 0 && ignore_errors {
        return Err(Message::from(
            SkippedDiagnostic.with_file_path(workspace_file.path.display().to_string()),
        ));
    }

    let printed = workspace_file
        .guard()
        .format_file()
        .with_file_path_and_code(
            workspace_file.path.display().to_string(),
            category!("format"),
        )?;

    let output = printed.into_code();

    // NOTE: ignoring the
    if ignore_errors {
        return Ok(FileStatus::Ignored);
    }

    if output != input {
        if should_write {
            workspace_file.update_file(output)?;
        } else {
            if !ctx.execution.should_report_to_terminal() {
                ctx.push_format_stat(
                    workspace_file.path.display().to_string(),
                    FormatterReportFileDetail {
                        formatted_content: Some(output.clone()),
                    },
                )
            }

            return Ok(FileStatus::Message(Message::Diff {
                file_name: workspace_file.path.display().to_string(),
                old: input,
                new: output,
                diff_kind: DiffKind::Format,
            }));
        }
    }
    Ok(FileStatus::Success)
}
