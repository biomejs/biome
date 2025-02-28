use crate::execute::diagnostics::{ResultExt, SearchDiagnostic};
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_diagnostics::{category, DiagnosticExt};
use biome_fs::{BiomePath, TraversalContext};
use biome_service::diagnostics::FileTooLarge;
use biome_service::workspace::PatternId;

pub(crate) fn search<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
    pattern: &PatternId,
) -> FileResult {
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    let result = workspace_file.guard().check_file_size()?;
    if result.is_too_large() {
        ctx.push_diagnostic(
            FileTooLarge::from(result)
                .with_file_path(workspace_file.path.to_string())
                .with_category(category!("search")),
        );
        Ok(FileStatus::Ignored)
    } else {
        search_with_guard(ctx, &mut workspace_file, pattern)
    }
}

pub(crate) fn search_with_guard<'ctx>(
    _ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
    pattern: &PatternId,
) -> FileResult {
    let _ = tracing::info_span!("Search ", path =? workspace_file.path).entered();
    let result = workspace_file
        .guard()
        .search_pattern(pattern)
        .with_file_path_and_code(workspace_file.path.to_string(), category!("search"))?;

    let input = workspace_file.input()?;
    let file_name = workspace_file.path.to_string();
    let matches_len = result.matches.len();

    let search_results = Message::Diagnostics {
        file_path: file_name,
        content: input,
        diagnostics: result
            .matches
            .into_iter()
            .map(|mat| SearchDiagnostic.with_file_span(mat))
            .collect(),
        skipped_diagnostics: 0,
    };

    Ok(FileStatus::SearchResult(matches_len, search_results))
}
