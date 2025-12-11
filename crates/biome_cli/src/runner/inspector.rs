use crate::TraversalMode;
use crate::runner::crawler::{CrawlerContext, CrawlerOptions};
use crate::runner::execution::Execution;
use crate::runner::process_file::{FileStatus, Message, ProcessFile};
use biome_diagnostics::{DiagnosticExt, DiagnosticTags, category};
use biome_fs::{BiomePath, FileSystem, TraversalContext};
use biome_service::file_handlers::DocumentFileSource;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    FileFeaturesResult, IgnoreKind, PathIsIgnoredParams, SupportsFeatureParams,
};
use biome_service::{Workspace, WorkspaceError, extension_error};
use std::panic::catch_unwind;

/// Path entries that we want to ignore during the OS traversal.
pub const TRAVERSAL_IGNORE_ENTRIES: &[&[u8]] = &[
    b".git",
    b".hg",
    b".svn",
    b".yarn",
    b".DS_Store",
    b"node_modules",
];

pub trait Inspector: Default + Send + Sync + 'static {
    type ProcessFile: ProcessFile;

    fn can_handle<Ctx>(&self, biome_path: &BiomePath, ctx: &Ctx) -> bool
    where
        Ctx: CrawlerContext,
    {
        if biome_path
            .file_name()
            .is_some_and(|file_name| TRAVERSAL_IGNORE_ENTRIES.contains(&file_name.as_bytes()))
        {
            return false;
        }
        let fs = ctx.fs();
        let workspace = ctx.workspace();
        let execution = ctx.execution();
        let path = biome_path.as_path();
        let project_key = ctx.project_key();
        if fs.path_is_dir(path) || fs.path_is_symlink(path) {
            // handle:
            // - directories
            // - symlinks
            // - unresolved symlinks
            //   e.g `symlink/subdir` where symlink points to a directory that includes `subdir`.
            //   Note that `symlink/subdir` is not an existing file.
            let can_handle = !workspace
                .is_path_ignored(PathIsIgnoredParams {
                    project_key,
                    path: biome_path.clone(),
                    features: execution.to_feature(),
                    ignore_kind: IgnoreKind::Ancestors,
                })
                .unwrap_or_else(|err| {
                    ctx.push_diagnostic(err.into());
                    false
                });

            return can_handle;
        }

        // bail on fifo and socket files
        if !fs.path_is_file(path) {
            return false;
        }

        let file_features = workspace.file_features(SupportsFeatureParams {
            project_key,
            path: biome_path.clone(),
            features: execution.to_feature(),
            inline_config: None,
        });

        let can_read = DocumentFileSource::can_read(biome_path);

        let file_features = match file_features {
            Ok(FileFeaturesResult {
                features_supported: file_features,
            }) => {
                if file_features.is_protected() {
                    ctx.push_diagnostic(
                        WorkspaceError::protected_file(biome_path.to_string()).into(),
                    );
                    return false;
                }

                if file_features.is_not_supported() && !file_features.is_ignored() && !can_read {
                    // we should throw a diagnostic if we can't handle a file that isn't ignored
                    miss_handler_err(ctx, fs, extension_error(biome_path), biome_path);
                    return false;
                }
                file_features
            }
            Err(err) => {
                miss_handler_err(ctx, fs, err, biome_path);
                return false;
            }
        };

        execution.can_handle(file_features)
    }

    fn handle_path<Ctx>(&self, biome_path: &BiomePath, ctx: &Ctx)
    where
        Ctx: CrawlerContext,
    {
        // ProcessFile::process_file is generic over Ctx: TraversalContext
        // We pass &Ctx which should also implement TraversalContext
        match Self::ProcessFile::process_file(ctx, biome_path.clone()) {
            Ok(FileStatus::Changed) => {
                ctx.increment_changed(biome_path);
            }
            Ok(FileStatus::Unchanged) => {
                ctx.increment_unchanged();
            }
            Ok(FileStatus::SearchResult(num_matches, msg)) => {
                ctx.increment_unchanged();
                ctx.increment_matches(num_matches);
                ctx.push_message(msg);
            }
            Ok(FileStatus::Message(msg)) => {
                ctx.increment_unchanged();
                ctx.push_message(msg);
            }
            Ok(FileStatus::Protected(file_path)) => {
                ctx.increment_unchanged();
                ctx.push_diagnostic(WorkspaceError::protected_file(file_path).into());
            }
            Ok(FileStatus::Ignored) => {}
            Err(err) => {
                ctx.increment_unchanged();
                // TODO: add skipped counter to CrawlerContext
                ctx.push_message(err);
            }
        }
    }
}

pub(crate) fn miss_handler_err(
    ctx: &dyn TraversalContext,
    fs: &dyn FileSystem,
    err: WorkspaceError,
    biome_path: &BiomePath,
) {
    let file_path = fs
        .working_directory()
        .as_ref()
        .and_then(|wd| {
            biome_path
                .strip_prefix(wd)
                .ok()
                .map(|path| path.to_string())
        })
        .unwrap_or(biome_path.to_string());
    ctx.push_diagnostic(
        err.with_category(category!("files/missingHandler"))
            .with_file_path(file_path)
            .with_tags(DiagnosticTags::VERBOSE),
    );
}
