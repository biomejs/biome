use crate::execute::process_file::format::format_with_guard;
use crate::execute::process_file::lint::lint_with_guard;
use crate::execute::process_file::organize_imports::organize_imports_with_guard;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{FileResult, FileStatus, Message, SharedTraversalOptions};
use crate::CliDiagnostic;
use rome_diagnostics::Category;
use rome_service::workspace::{FeatureName, FileFeaturesResult};
use std::path::Path;

pub(crate) fn check_file<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: &Path,
    file_features: &'ctx FileFeaturesResult,
    category: &'static Category,
) -> FileResult {
    let mut has_errors = false;
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;

    if file_features.supports_for(&FeatureName::Lint) {
        let lint_result = lint_with_guard(ctx, &mut workspace_file);
        match lint_result {
            Ok(status) => {
                if let FileStatus::Message(msg) = status {
                    if msg.is_diagnostic() {
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
    if file_features.supports_for(&FeatureName::OrganizeImports) {
        let organize_imports_result = organize_imports_with_guard(ctx, &mut workspace_file);
        match organize_imports_result {
            Ok(status) => {
                if let FileStatus::Message(msg) = status {
                    if msg.is_diagnostic() {
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

    if file_features.supports_for(&FeatureName::Format) {
        let format_result = format_with_guard(ctx, &mut workspace_file);
        match format_result {
            Ok(status) => {
                if let FileStatus::Message(msg) = status {
                    if msg.is_diagnostic() {
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
        if ctx.execution.is_check_apply() || ctx.execution.is_check_apply_unsafe() {
            Ok(FileStatus::Message(Message::ApplyError(
                CliDiagnostic::file_check_apply_error(path.display().to_string(), category),
            )))
        } else {
            Ok(FileStatus::Message(Message::ApplyError(
                CliDiagnostic::file_check_error(path.display().to_string(), category),
            )))
        }
    } else {
        Ok(FileStatus::Success)
    }
}
