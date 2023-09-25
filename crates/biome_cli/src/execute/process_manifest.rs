use crate::execute::diagnostics::{ResultExt, ResultIoExt};
use crate::execute::process_file::{FileResult, FileStatus, Message};
use crate::execute::traverse::TraversalOptions;
use biome_diagnostics::{category, Error};
use biome_fs::{OpenOptions, RomePath};
use biome_service::file_handlers::Language;
use biome_service::workspace::{FileGuard, OpenFileParams, RuleCategories};
use std::path::Path;
use std::sync::atomic::Ordering;

pub(crate) fn process_manifest(ctx: &TraversalOptions, path: &Path) -> FileResult {
    tracing::trace_span!("process_manifest", path = ?path).in_scope(move || {
        let biome_path = RomePath::new(path);
        let open_options = OpenOptions::default()
            .read(true)
            .write(ctx.execution.requires_write_access());
        let mut file = ctx
            .fs
            .open_with_options(path, open_options)
            .with_file_path(path.display().to_string())?;

        let mut input = String::new();
        file.read_to_string(&mut input)
            .with_file_path(path.display().to_string())?;
        ctx.increment_processed();
        if ctx.execution.is_check() {}

        let file_guard = FileGuard::open(
            ctx.workspace,
            OpenFileParams {
                path: biome_path.clone(),
                version: 0,
                content: input.clone(),
                language_hint: Language::default(),
            },
        )
        .with_file_path_and_code(path.display().to_string(), category!("internalError/fs"))?;

        let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
        if ctx.execution.is_check() {
            let result = file_guard
                .pull_diagnostics(RuleCategories::LINT, max_diagnostics.into())
                .with_file_path_and_code(path.display().to_string(), category!("lint"))?;

            if result.errors > 0 {
                return Ok(FileStatus::Message(Message::Diagnostics {
                    name: path.display().to_string(),
                    content: input.clone(),
                    diagnostics: result.diagnostics.into_iter().map(Error::from).collect(),
                    skipped_diagnostics: result.skipped_diagnostics,
                }));
            }
        }

        Ok(FileStatus::Ignored)
    })
}
