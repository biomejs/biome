use crate::execute::diagnostics::ResultExt;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{FileResult, FileStatus, SharedTraversalOptions};
use biome_diagnostics::category;
use biome_service::workspace::RuleCategories;
use std::path::Path;
use std::sync::atomic::Ordering;
use tracing::debug;

pub(crate) fn search<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: &Path,
    pattern: String,
) -> FileResult {
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    search_with_guard(ctx, &mut workspace_file, pattern)
}

pub(crate) fn search_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
    pattern: String,
) -> FileResult {
    tracing::info_span!("Processes searching", path =? workspace_file.path.display()).in_scope(
        move || {
            let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
            debug!("Pulling diagnostics from parsed file");
            let diagnostics_result = workspace_file
                .guard()
                .pull_diagnostics(RuleCategories::SYNTAX, max_diagnostics.into())
                .with_file_path_and_code(
                    workspace_file.path.display().to_string(),
                    category!("format"),
                )?;

            let input = workspace_file.input()?;

            let printed = workspace_file
                .guard()
                .search_pattern(pattern)
                .with_file_path_and_code(
                    workspace_file.path.display().to_string(),
                    category!("search"),
                )?;

            // FIXME: We need to report some real results here...
            Ok(FileStatus::Unchanged)
        },
    )
}
