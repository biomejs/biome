use std::ffi::OsStr;

use crate::execute::diagnostics::ResultExt;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{
    DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions,
};
use biome_analyze::RuleCategoriesBuilder;
use biome_diagnostics::category;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::workspace::FixFileMode;

/// Lints a single file and returns a [FileResult]
pub(crate) fn assists_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    let _ =
        tracing::info_span!("Processes assists", path =? workspace_file.path.display()).entered();
    let input = workspace_file.input()?;

    let only = Vec::new();
    let skip = Vec::new();
    let fix_result = workspace_file
        .guard()
        .fix_file(
            FixFileMode::SafeFixes,
            false,
            RuleCategoriesBuilder::default().with_action().build(),
            only.clone(),
            skip.clone(),
        )
        .with_file_path_and_code(
            workspace_file.path.display().to_string(),
            category!("assists"),
        )?;

    ctx.push_message(Message::SkippedFixes {
        skipped_suggested_fixes: fix_result.skipped_suggested_fixes,
    });

    let mut output = fix_result.code;

    match workspace_file.as_extension().map(OsStr::as_encoded_bytes) {
        Some(b"astro") => {
            output = AstroFileHandler::output(input.as_str(), output.as_str());
        }
        Some(b"vue") => {
            output = VueFileHandler::output(input.as_str(), output.as_str());
        }
        Some(b"svelte") => {
            output = SvelteFileHandler::output(input.as_str(), output.as_str());
        }
        _ => {}
    }
    if input != output {
        if ctx.execution.as_fix_file_mode().is_none() {
            return Ok(FileStatus::Message(Message::Diff {
                file_name: workspace_file.path.display().to_string(),
                old: input,
                new: output,
                diff_kind: DiffKind::Assists,
            }));
        } else {
            if output != input && ctx.execution.as_fix_file_mode().is_some() {
                workspace_file.update_file(output)?;
            }
            Ok(FileStatus::Changed)
        }
    } else {
        Ok(FileStatus::Unchanged)
    }
}
