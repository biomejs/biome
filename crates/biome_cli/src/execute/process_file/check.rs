use crate::execute::process_file::format::format_with_guard;
use crate::execute::process_file::lint::lint_with_guard;
use crate::execute::process_file::organize_imports::organize_imports_with_guard;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_service::workspace::FileFeaturesResult;
use std::path::Path;

pub(crate) fn check_file<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: &Path,
    file_features: &'ctx FileFeaturesResult,
) -> FileResult {
    let mut has_errors = false;
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    let mut changed = false;
    tracing::info_span!("Process check", path =? workspace_file.path.display()).in_scope(
        move || {
            if file_features.supports_lint() {
                let lint_result = lint_with_guard(ctx, &mut workspace_file);
                match lint_result {
                    Ok(status) => {
                        changed = status.is_changed();
                        if let FileStatus::Message(msg) = status {
                            if msg.is_error() {
                                has_errors = true
                            }
                            ctx.push_message(msg);
                        }
                    }
                    Err(err) => {
                        ctx.push_message(err);
                        has_errors = true;
                    }
                }
            }
            if file_features.supports_organize_imports() {
                let organize_imports_result = organize_imports_with_guard(ctx, &mut workspace_file);
                match organize_imports_result {
                    Ok(status) => {
                        changed = status.is_changed();
                        if let FileStatus::Message(msg) = status {
                            if msg.is_error() {
                                has_errors = true
                            }
                            ctx.push_message(msg);
                        }
                    }
                    Err(err) => {
                        ctx.push_message(err);
                        has_errors = true;
                    }
                }
            }

            if file_features.supports_format() {
                let format_result = format_with_guard(ctx, &mut workspace_file);
                match format_result {
                    Ok(status) => {
                        changed = status.is_changed();
                        if let FileStatus::Message(msg) = status {
                            if msg.is_error() {
                                has_errors = true
                            }
                            ctx.push_message(msg);
                        }
                    }
                    Err(err) => {
                        ctx.push_message(err);
                        has_errors = true;
                    }
                }
            }

            if has_errors {
                Ok(FileStatus::Message(Message::Failure))
            } else if changed {
                Ok(FileStatus::Changed)
            } else {
                Ok(FileStatus::Unchanged)
            }
        },
    )
}
