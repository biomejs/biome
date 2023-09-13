use crate::execute::diagnostics::ResultExt;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{
    DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions,
};
use biome_diagnostics::category;

/// Lints a single file and returns a [FileResult]
pub(crate) fn organize_imports_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    let sorted = workspace_file
        .guard()
        .organize_imports()
        .with_file_path_and_code(
            workspace_file.path.display().to_string(),
            category!("organizeImports"),
        )?;

    let input = workspace_file.input()?;
    if sorted.code != input {
        if ctx.execution.is_check_apply()
            || ctx.execution.is_check_apply_unsafe()
            || ctx.execution.is_check_apply()
        {
            workspace_file.update_file(sorted.code)?;
        } else {
            return Ok(FileStatus::Message(Message::Diff {
                file_name: workspace_file.path.display().to_string(),
                old: input,
                new: sorted.code,
                diff_kind: DiffKind::OrganizeImports,
            }));
        }
    }

    Ok(FileStatus::Success)
}
