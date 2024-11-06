use crate::execute::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{
    DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions,
};
use crate::execute::TraversalMode;
use biome_analyze::RuleCategoriesBuilder;
use biome_diagnostics::{category, Diagnostic, DiagnosticExt, Error, Severity};
use biome_fs::TraversalContext;
use biome_service::diagnostics::FileTooLarge;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use std::ffi::OsStr;
use std::path::Path;
use std::sync::atomic::Ordering;
use tracing::debug;

pub(crate) fn format<'ctx>(ctx: &'ctx SharedTraversalOptions<'ctx, '_>, path: &Path) -> FileResult {
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    let result = workspace_file.guard().check_file_size()?;
    if result.is_too_large() {
        ctx.push_diagnostic(
            FileTooLarge::from(result)
                .with_file_path(path.display().to_string())
                .with_category(category!("format")),
        );
        Ok(FileStatus::Ignored)
    } else {
        format_with_guard(ctx, &mut workspace_file)
    }
}

pub(crate) fn format_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    let _ = tracing::info_span!("Format", path =? workspace_file.path.display()).entered();
    let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
    let diagnostics_result = workspace_file
        .guard()
        .pull_diagnostics(
            RuleCategoriesBuilder::default().with_syntax().build(),
            max_diagnostics,
            Vec::new(),
            Vec::new(),
        )
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

    match workspace_file.as_extension().map(OsStr::as_encoded_bytes) {
        Some(b"astro") => {
            if output.is_empty() {
                return Ok(FileStatus::Unchanged);
            }
            output = AstroFileHandler::output(input.as_str(), output.as_str());
        }
        Some(b"vue") => {
            if output.is_empty() {
                return Ok(FileStatus::Unchanged);
            }
            output = VueFileHandler::output(input.as_str(), output.as_str());
        }

        Some(b"svelte") => {
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
            Ok(FileStatus::Changed)
        } else {
            Ok(FileStatus::Message(Message::Diff {
                file_name: workspace_file.path.display().to_string(),
                old: input,
                new: output,
                diff_kind: DiffKind::Format,
            }))
        }
    } else {
        Ok(FileStatus::Unchanged)
    }
}
