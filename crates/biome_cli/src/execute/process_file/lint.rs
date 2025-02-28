use crate::execute::diagnostics::ResultExt;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{FileResult, FileStatus, Message, SharedTraversalOptions};
use crate::TraversalMode;
use biome_analyze::RuleCategoriesBuilder;
use biome_diagnostics::{category, DiagnosticExt, Error};
use biome_fs::{BiomePath, TraversalContext};
use biome_rowan::TextSize;
use biome_service::diagnostics::FileTooLarge;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use std::sync::atomic::Ordering;
use tracing::{info, instrument};

/// Lints a single file and returns a [FileResult]
#[instrument(level = "debug", name = "cli_lint", skip_all)]
pub(crate) fn lint<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
    suppress: bool,
    suppression_reason: Option<&str>,
) -> FileResult {
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    let result = workspace_file.guard().check_file_size()?;
    if result.is_too_large() {
        ctx.push_diagnostic(
            FileTooLarge::from(result)
                .with_file_path(workspace_file.path.to_string())
                .with_category(category!("lint")),
        );
        Ok(FileStatus::Ignored)
    } else {
        lint_with_guard(ctx, &mut workspace_file, suppress, suppression_reason)
    }
}

#[instrument(level = "debug", name = "cli_lint_guard", skip_all)]

pub(crate) fn lint_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
    suppress: bool,
    suppression_reason: Option<&str>,
) -> FileResult {
    let mut input = workspace_file.input()?;
    let mut changed = false;
    let (only, skip) =
        if let TraversalMode::Lint { only, skip, .. } = ctx.execution.traversal_mode() {
            (only.clone(), skip.clone())
        } else {
            (Vec::new(), Vec::new())
        };
    if let Some(fix_mode) = ctx.execution.as_fix_file_mode() {
        let suppression_explanation = if suppress && suppression_reason.is_none() {
            "ignored using `--suppress`"
        } else {
            suppression_reason.unwrap_or("<explanation>")
        };

        let fix_result = workspace_file
            .guard()
            .fix_file(
                *fix_mode,
                false,
                RuleCategoriesBuilder::default()
                    .with_syntax()
                    .with_lint()
                    .build(),
                only.clone(),
                skip.clone(),
                Some(suppression_explanation.to_string()),
            )
            .with_file_path_and_code(workspace_file.path.to_string(), category!("lint"))?;

        info!(
            "Fix file summary result. Errors {}, skipped fixes {}, actions {}",
            fix_result.errors,
            fix_result.skipped_suggested_fixes,
            fix_result.actions.len()
        );

        ctx.push_message(Message::SkippedFixes {
            skipped_suggested_fixes: fix_result.skipped_suggested_fixes,
        });

        let mut output = fix_result.code;

        match workspace_file.as_extension() {
            Some("astro") => {
                output = AstroFileHandler::output(input.as_str(), output.as_str());
            }
            Some("vue") => {
                output = VueFileHandler::output(input.as_str(), output.as_str());
            }
            Some("svelte") => {
                output = SvelteFileHandler::output(input.as_str(), output.as_str());
            }
            _ => {}
        }
        if output != input {
            changed = true;
            workspace_file.update_file(output)?;
            input = workspace_file.input()?;
        }
    }

    let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
    let pull_diagnostics_result = workspace_file
        .guard()
        .pull_diagnostics(
            RuleCategoriesBuilder::default()
                .with_syntax()
                .with_lint()
                .build(),
            max_diagnostics,
            only,
            skip,
        )
        .with_file_path_and_code(workspace_file.path.to_string(), category!("lint"))?;

    let no_diagnostics = pull_diagnostics_result.diagnostics.is_empty()
        && pull_diagnostics_result.skipped_diagnostics == 0;

    if !no_diagnostics {
        let offset = match workspace_file.as_extension() {
            Some("vue") => VueFileHandler::start(input.as_str()),
            Some("astro") => AstroFileHandler::start(input.as_str()),
            Some("svelte") => SvelteFileHandler::start(input.as_str()),
            _ => None,
        };

        ctx.push_message(Message::Diagnostics {
            file_path: workspace_file.path.to_string(),
            content: input,
            diagnostics: pull_diagnostics_result
                .diagnostics
                .into_iter()
                .map(|d| {
                    if let Some(offset) = offset {
                        d.with_offset(TextSize::from(offset))
                    } else {
                        d
                    }
                })
                .map(Error::from)
                .collect(),
            skipped_diagnostics: pull_diagnostics_result.skipped_diagnostics as u32,
        });
    }

    if changed {
        Ok(FileStatus::Changed)
    } else {
        Ok(FileStatus::Unchanged)
    }
}
