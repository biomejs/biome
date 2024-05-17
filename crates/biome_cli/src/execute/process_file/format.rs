use crate::execute::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{
    DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions,
};
use crate::execute::TraversalMode;
use biome_diagnostics::{category, Diagnostic, DiagnosticExt, Error, Severity};
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::workspace::RuleCategories;
use std::path::Path;
use std::sync::atomic::Ordering;
use tracing::debug;

pub(crate) fn format<'ctx>(ctx: &'ctx SharedTraversalOptions<'ctx, '_>, path: &Path) -> FileResult {
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    format_with_guard(ctx, &mut workspace_file)
}

pub(crate) fn format_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    tracing::info_span!("Processes formatting", path =? workspace_file.path.display()).in_scope(
        move || {
            let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
            debug!("Pulling diagnostics from parsed file");
            let diagnostics_result = workspace_file
                .guard()
                .pull_diagnostics(RuleCategories::SYNTAX, max_diagnostics.into(), None)
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
            debug!("Should write the file to disk? {}", should_write);
            debug!("Should ignore errors? {}", ignore_errors);

            if diagnostics_result.errors > 0 && ignore_errors {
                return Err(Message::from(
                    SkippedDiagnostic.with_file_path(workspace_file.path.display().to_string()),
                ));
            }

            ctx.push_message(Message::Diagnostics {
                name: workspace_file.path.display().to_string(),
                content: input.clone(),
                diagnostics: diagnostics_result
                    .diagnostics
                    .into_iter()
                    .filter_map(|diag| {
                        if diag.severity() >= Severity::Error && ignore_errors {
                            None
                        } else {
                            Some(Error::from(diag))
                        }
                    })
                    .collect(),
                skipped_diagnostics: diagnostics_result.skipped_diagnostics as u32,
            });

            let printed = workspace_file
                .guard()
                .format_file()
                .with_file_path_and_code(
                    workspace_file.path.display().to_string(),
                    category!("format"),
                )?;

            let mut output = printed.into_code();

            if ignore_errors {
                return Ok(FileStatus::Ignored);
            }

            match workspace_file.as_extension() {
                Some("astro") => {
                    if output.is_empty() {
                        return Ok(FileStatus::Unchanged);
                    }
                    output = AstroFileHandler::output(input.as_str(), output.as_str());
                }
                Some("vue") => {
                    if output.is_empty() {
                        return Ok(FileStatus::Unchanged);
                    }
                    output = VueFileHandler::output(input.as_str(), output.as_str());
                }

                Some("svelte") => {
                    if output.is_empty() {
                        return Ok(FileStatus::Unchanged);
                    }
                    output = SvelteFileHandler::output(input.as_str(), output.as_str());
                }
                _ => {}
            }

            if output != input {
                if should_write {
                    workspace_file.update_file(output)?;
                } else {
                    return Ok(FileStatus::Message(Message::Diff {
                        file_name: workspace_file.path.display().to_string(),
                        old: input,
                        new: output,
                        diff_kind: DiffKind::Format,
                    }));
                }
                Ok(FileStatus::Changed)
            } else {
                Ok(FileStatus::Unchanged)
            }
        },
    )
}
