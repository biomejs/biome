use std::ffi::OsStr;

use crate::execute::diagnostics::ResultExt;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{
    DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions,
};
use biome_diagnostics::category;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};

/// Lints a single file and returns a [FileResult]
pub(crate) fn organize_imports_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    let _ =
        tracing::info_span!("Sort imports for ", path =? workspace_file.path.display()).entered();
    let sorted = workspace_file
        .guard()
        .organize_imports()
        .with_file_path_and_code(
            workspace_file.path.display().to_string(),
            category!("organizeImports"),
        )?;

    let input = workspace_file.input()?;
    let mut output = sorted.code;

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
        if ctx.execution.is_check_apply() || ctx.execution.is_check_apply_unsafe() {
            workspace_file.update_file(output)?;
        } else {
            return Ok(FileStatus::Message(Message::Diff {
                file_name: workspace_file.path.display().to_string(),
                old: input,
                new: output,
                diff_kind: DiffKind::OrganizeImports,
            }));
        }
        Ok(FileStatus::Changed)
    } else {
        Ok(FileStatus::Unchanged)
    }
}
