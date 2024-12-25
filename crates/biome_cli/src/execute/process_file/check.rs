use crate::execute::process_file::assist::assist_with_guard;
use crate::execute::process_file::format::format_with_guard;
use crate::execute::process_file::lint::lint_with_guard;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_diagnostics::{category, DiagnosticExt};
use biome_fs::{BiomePath, TraversalContext};
use biome_service::diagnostics::FileTooLarge;
use biome_service::workspace::FileFeaturesResult;

pub(crate) fn check_file<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
    file_features: &'ctx FileFeaturesResult,
) -> FileResult {
    let mut has_failures = false;
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    let result = workspace_file.guard().check_file_size()?;
    if result.is_too_large() {
        ctx.push_diagnostic(
            FileTooLarge::from(result)
                .with_file_path(workspace_file.path.to_string())
                .with_category(category!("check")),
        );
        return Ok(FileStatus::Ignored);
    }
    let mut changed = false;
    let _ = tracing::info_span!("Check ", path =? workspace_file.path).entered();
    if file_features.supports_lint() {
        let lint_result = lint_with_guard(ctx, &mut workspace_file, false, None);
        match lint_result {
            Ok(status) => {
                if status.is_changed() {
                    changed = true
                }
                if let FileStatus::Message(msg) = status {
                    if msg.is_failure() {
                        has_failures = true;
                    }
                    ctx.push_message(msg);
                }
            }
            Err(err) => {
                ctx.push_message(err);
                has_failures = true;
            }
        }
    }

    if file_features.supports_assist() {
        let assist_result = assist_with_guard(ctx, &mut workspace_file);
        match assist_result {
            Ok(status) => {
                if status.is_changed() {
                    changed = true
                }
                if let FileStatus::Message(msg) = status {
                    if msg.is_failure() {
                        has_failures = true;
                    }
                    ctx.push_message(msg);
                }
            }
            Err(err) => {
                ctx.push_message(err);
                has_failures = true;
            }
        }
    }

    if file_features.supports_format() {
        let format_result = format_with_guard(ctx, &mut workspace_file);
        match format_result {
            Ok(status) => {
                if status.is_changed() {
                    changed = true
                }
                if let FileStatus::Message(msg) = status {
                    if msg.is_failure() {
                        has_failures = true;
                    }
                    ctx.push_message(msg);
                }
            }
            Err(err) => {
                ctx.push_message(err);
                has_failures = true;
            }
        }
    }

    if has_failures {
        Ok(FileStatus::Message(Message::Failure))
    } else if changed {
        Ok(FileStatus::Changed)
    } else {
        Ok(FileStatus::Unchanged)
    }
}
