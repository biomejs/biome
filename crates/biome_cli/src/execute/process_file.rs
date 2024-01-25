mod check;
mod format;
mod lint;
mod organize_imports;
pub(crate) mod workspace_file;

use crate::execute::diagnostics::{ResultExt, UnhandledDiagnostic};
use crate::execute::process_file::check::check_file;
use crate::execute::process_file::format::format;
use crate::execute::process_file::lint::lint;
use crate::execute::traverse::TraversalOptions;
use crate::execute::TraversalMode;
use crate::CliDiagnostic;
use biome_diagnostics::{category, DiagnosticExt, DiagnosticTags, Error};
use biome_fs::RomePath;
use biome_service::workspace::{FeatureName, FeaturesBuilder, SupportKind, SupportsFeatureParams};
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::Path;

#[derive(Debug)]
pub(crate) enum FileStatus {
    Success,
    Message(Message),
    Ignored,
    /// Files that belong to other tools and shouldn't be touched
    Protected(String),
}

/// Wrapper type for messages that can be printed during the traversal process
#[derive(Debug)]
pub(crate) enum Message {
    SkippedFixes {
        /// Suggested fixes skipped during the lint traversal
        skipped_suggested_fixes: u32,
    },
    ApplyError(CliDiagnostic),
    Error(Error),
    Diagnostics {
        name: String,
        content: String,
        diagnostics: Vec<Error>,
        skipped_diagnostics: u64,
    },
    Diff {
        file_name: String,
        old: String,
        new: String,
        diff_kind: DiffKind,
    },
}

impl Message {
    pub(crate) const fn is_diagnostic(&self) -> bool {
        matches!(self, Message::Diff { .. } | Message::Diagnostics { .. })
    }
}

#[derive(Debug)]
pub(crate) enum DiffKind {
    Format,
    OrganizeImports,
}

impl<D> From<D> for Message
where
    Error: From<D>,
    D: std::fmt::Debug,
{
    fn from(err: D) -> Self {
        Self::Error(Error::from(err))
    }
}

/// The return type for [process_file], with the following semantics:
/// - `Ok(Success)` means the operation was successful (the file is added to
///   the `processed` counter)
/// - `Ok(Message(_))` means the operation was successful but a message still
///   needs to be printed (eg. the diff when not in CI or write mode)
/// - `Ok(Ignored)` means the file was ignored (the file is not added to the
///   `processed` or `skipped` counters)
/// - `Err(_)` means the operation failed and the file should be added to the
///   `skipped` counter
pub(crate) type FileResult = Result<FileStatus, Message>;

/// Data structure that allows to pass [TraversalOptions] to multiple consumers, bypassing the
/// compiler constraints set by the lifetimes of the [TraversalOptions]
pub(crate) struct SharedTraversalOptions<'ctx, 'app> {
    inner: &'app TraversalOptions<'ctx, 'app>,
    _p: PhantomData<&'app ()>,
}

impl<'ctx, 'app> SharedTraversalOptions<'ctx, 'app> {
    fn new(t: &'app TraversalOptions<'ctx, 'app>) -> Self {
        Self {
            _p: PhantomData,
            inner: t,
        }
    }
}

impl<'ctx, 'app> Deref for SharedTraversalOptions<'ctx, 'app> {
    type Target = TraversalOptions<'ctx, 'app>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

/// This function performs the actual processing: it reads the file from disk
/// and parse it; analyze and / or format it; then it either fails if error
/// diagnostics were emitted, or compare the formatted code with the original
/// content of the file and emit a diff or write the new content to the disk if
/// write mode is enabled
pub(crate) fn process_file(ctx: &TraversalOptions, path: &Path) -> FileResult {
    tracing::trace_span!("process_file", path = ?path).in_scope(move || {
        let rome_path = RomePath::new(path);
        let file_features = ctx
            .workspace
            .file_features(SupportsFeatureParams {
                path: rome_path,
                feature: FeaturesBuilder::new()
                    .with_formatter()
                    .with_linter()
                    .with_organize_imports()
                    .build(),
            })
            .with_file_path_and_code_and_tags(
                path.display().to_string(),
                category!("files/missingHandler"),
                DiagnosticTags::VERBOSE,
            )?;

        // first we stop if there are some files that don't have ALL features enabled, e.g. images, fonts, etc.
        if file_features.is_ignored() || file_features.is_not_enabled() {
            return Ok(FileStatus::Ignored);
        } else if file_features.is_not_supported() {
            return Err(Message::from(
                UnhandledDiagnostic.with_file_path(path.display().to_string()),
            ));
        }

        // then we pick the specific features for this file
        let unsupported_reason = match ctx.execution.traversal_mode() {
            TraversalMode::Check { .. } => file_features
                .support_kind_for(&FeatureName::Lint)
                .and_then(|support_kind| {
                    if support_kind.is_not_enabled() {
                        Some(support_kind)
                    } else {
                        None
                    }
                })
                .and(
                    file_features
                        .support_kind_for(&FeatureName::Format)
                        .and_then(|support_kind| {
                            if support_kind.is_not_enabled() {
                                Some(support_kind)
                            } else {
                                None
                            }
                        }),
                )
                .and(
                    file_features
                        .support_kind_for(&FeatureName::OrganizeImports)
                        .and_then(|support_kind| {
                            if support_kind.is_not_enabled() {
                                Some(support_kind)
                            } else {
                                None
                            }
                        }),
                ),
            TraversalMode::CI { .. } => file_features
                .support_kind_for(&FeatureName::Lint)
                .and_then(|support_kind| {
                    if support_kind.is_not_enabled() {
                        Some(support_kind)
                    } else {
                        None
                    }
                })
                .and(
                    file_features
                        .support_kind_for(&FeatureName::Format)
                        .and_then(|support_kind| {
                            if support_kind.is_not_enabled() {
                                Some(support_kind)
                            } else {
                                None
                            }
                        }),
                )
                .and(
                    file_features
                        .support_kind_for(&FeatureName::OrganizeImports)
                        .and_then(|support_kind| {
                            if support_kind.is_not_enabled() {
                                Some(support_kind)
                            } else {
                                None
                            }
                        }),
                ),
            TraversalMode::Format { .. } => file_features.support_kind_for(&FeatureName::Format),
            TraversalMode::Lint { .. } => file_features.support_kind_for(&FeatureName::Lint),
            TraversalMode::Migrate { .. } => None,
        };

        if let Some(reason) = unsupported_reason {
            match reason {
                SupportKind::FileNotSupported => {
                    return Err(Message::from(
                        UnhandledDiagnostic.with_file_path(path.display().to_string()),
                    ))
                }
                SupportKind::FeatureNotEnabled | SupportKind::Ignored => {
                    return Ok(FileStatus::Ignored)
                }
                SupportKind::Protected => {
                    return Ok(FileStatus::Protected(path.display().to_string()))
                }
                SupportKind::Supported => {}
            };
        }

        let shared_context = &SharedTraversalOptions::new(ctx);
        ctx.increment_processed();

        match ctx.execution.traversal_mode {
            TraversalMode::Lint { .. } => {
                // the unsupported case should be handled already at this point
                lint(shared_context, path)
            }
            TraversalMode::Format { .. } => {
                // the unsupported case should be handled already at this point
                format(shared_context, path)
            }
            TraversalMode::Check { .. } => {
                check_file(shared_context, path, &file_features, category!("check"))
            }
            TraversalMode::CI { .. } => {
                check_file(shared_context, path, &file_features, category!("ci"))
            }
            TraversalMode::Migrate { .. } => {
                unreachable!("The migration should not be called for this file")
            }
        }
    })
}
