use crate::execute::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{
    DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions,
};
use biome_analyze::RuleCategoriesBuilder;
use biome_diagnostics::{Diagnostic, DiagnosticExt, Error, Severity, category};
use biome_fs::{BiomePath, TraversalContext};
use biome_service::diagnostics::FileTooLarge;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use tracing::{debug, instrument};

#[instrument(name = "cli_format", level = "debug", skip(ctx, path))]
pub(crate) fn format<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    let result = workspace_file.guard().check_file_size()?;
    if result.is_too_large() {
        ctx.push_diagnostic(
            FileTooLarge::from(result)
                .with_file_path(workspace_file.path.to_string())
                .with_category(category!("format")),
        );
        Ok(FileStatus::Ignored)
    } else {
        format_with_guard(ctx, &mut workspace_file)
    }
}

#[instrument(level = "debug", skip(ctx, workspace_file))]
pub(crate) fn format_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    let diagnostics_result = workspace_file
        .guard()
        .pull_diagnostics(
            RuleCategoriesBuilder::default().with_syntax().build(),
            Vec::new(),
            Vec::new(),
            false, // NOTE: probably to revisit
        )
        .with_file_path_and_code(workspace_file.path.to_string(), category!("format"))?;

    let input = workspace_file.input()?;
    let should_write = ctx.execution.should_write();
    let skip_parse_errors = ctx.execution.should_skip_parse_errors();

    tracing::Span::current().record("should_write", tracing::field::display(&should_write));
    tracing::Span::current().record(
        "skip_parse_errors",
        tracing::field::display(&skip_parse_errors),
    );

    if diagnostics_result.errors > 0 && skip_parse_errors {
        ctx.push_message(Message::from(
            SkippedDiagnostic.with_file_path(workspace_file.path.to_string()),
        ));
        return Ok(FileStatus::Ignored);
    }

    ctx.push_message(Message::Diagnostics {
        file_path: workspace_file.path.to_string(),
        content: input.clone(),
        diagnostics: diagnostics_result
            .diagnostics
            .into_iter()
            // Formatting is usually blocked by errors, so we want to print only diagnostics that
            // Have error severity
            .filter_map(|diagnostic| {
                if diagnostic.severity() >= Severity::Error {
                    Some(Error::from(diagnostic))
                } else {
                    None
                }
            })
            .collect(),
        skipped_diagnostics: diagnostics_result.skipped_diagnostics as u32,
    });

    let printed = workspace_file
        .guard()
        .format_file()
        .with_file_path_and_code(workspace_file.path.to_string(), category!("format"))?;

    let mut output = printed.into_code();

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

    debug!("Format output is different from input: {}", output != input);
    if output != input {
        if should_write {
            workspace_file.update_file(output)?;
            Ok(FileStatus::Changed)
        } else {
            Ok(FileStatus::Message(Message::Diff {
                file_name: workspace_file.path.to_string(),
                old: input,
                new: output,
                diff_kind: DiffKind::Format,
            }))
        }
    } else {
        Ok(FileStatus::Unchanged)
    }
}
