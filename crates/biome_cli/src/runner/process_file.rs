use crate::CliDiagnostic;
use crate::cli_options::CliOptions;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{ResultExt, UnhandledDiagnostic};
use biome_console::Console;
use biome_diagnostics::serde::Diagnostic;
use biome_diagnostics::{DiagnosticExt, DiagnosticTags, Error, category};
use biome_fs::{BiomePath, FileSystem};
use biome_service::Workspace;
use biome_service::file_handlers::DocumentFileSource;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    CodeAction, FileFeaturesResult, SupportKind, SupportsFeatureParams,
};

#[derive(Debug)]
pub(crate) enum FileStatus {
    /// File changed and it was a success
    Changed,
    /// File unchanged, and it was a success
    Unchanged,
    /// While handling the file, something happened
    Message(Message),
    /// A match was found while searching a file
    SearchResult(usize, Message),
    /// File ignored, it should not be count as "handled"
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
    Failure,
    Error(Error),
    Diagnostics {
        file_path: String,
        content: String,
        diagnostics: Vec<Error>,
        skipped_diagnostics: u32,
    },
    DiagnosticsWithActions {
        file_path: String,
        content: String,
        diagnostics_with_actions: Vec<(Diagnostic, Vec<CodeAction>)>,
        skipped_diagnostics: u32,
    },
    Diff {
        file_name: String,
        old: String,
        new: String,
        diff_kind: DiffKind,
    },
}

impl Message {
    pub(crate) const fn is_failure(&self) -> bool {
        matches!(self, Self::Failure)
    }
}

#[derive(Debug)]
pub(crate) enum DiffKind {
    Format,
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

/// The return type for [crate::execute::process_file::process_file], with the following semantics:
/// - `Ok(Success)` means the operation was successful (the file is added to
///   the `processed` counter)
/// - `Ok(Message(_))` means the operation was successful but a message still
///   needs to be printed (eg. the diff when not in CI or write mode)
/// - `Ok(Ignored)` means the file was ignored (the file is not added to the
///   `processed` or `skipped` counters)
/// - `Err(_)` means the operation failed and the file should be added to the
///   `skipped` counter
pub(crate) type FileResult = Result<FileStatus, Message>;

pub(crate) struct ProcessStdinFilePayload<'a> {
    pub(crate) biome_path: &'a BiomePath,
    pub(crate) content: &'a str,
    pub(crate) fs: &'a dyn FileSystem,
    pub(crate) project_key: ProjectKey,
    pub(crate) workspace: &'a dyn Workspace,
    pub(crate) console: &'a mut dyn Console,
    pub(crate) cli_options: &'a CliOptions,
}

pub(crate) trait ProcessFile: Send + Sync {
    fn process_file<Ctx>(ctx: &Ctx, path: &BiomePath) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext;

    fn process_std_in(payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic>;

    fn execute<Ctx>(ctx: &Ctx, biome_path: &BiomePath) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        let FileFeaturesResult {
            features_supported: file_features,
        } = ctx
            .workspace()
            .file_features(SupportsFeatureParams {
                project_key: ctx.project_key(),
                path: biome_path.clone(),
                features: ctx.execution().to_feature(),
                inline_config: None,
            })
            .with_file_path_and_code_and_tags(
                biome_path.to_string(),
                category!("files/missingHandler"),
                DiagnosticTags::VERBOSE,
            )?;

        // first we stop if there are some files that don't have ALL features enabled, e.g. images, fonts, etc.
        if file_features.is_ignored() || file_features.is_not_enabled() {
            return Ok(FileStatus::Ignored);
        } else if file_features.is_not_supported() || !DocumentFileSource::can_read(biome_path) {
            return Err(Message::from(
                UnhandledDiagnostic.with_file_path(biome_path.to_string()),
            ));
        }

        // then we pick the specific features for this file
        let unsupported_reason = ctx.execution().supports_kind(&file_features);

        // TODO move logic
        // {
        //     TraversalMode::Check { .. } | TraversalMode::CI { .. } => file_features
        //         .support_kind_if_not_enabled(FeatureKind::Lint)
        //         .and(file_features.support_kind_if_not_enabled(FeatureKind::Format))
        //         .and(file_features.support_kind_if_not_enabled(FeatureKind::Assist)),
        //     TraversalMode::Format { .. } => {
        //         Some(file_features.support_kind_for(FeatureKind::Format))
        //     }
        //     TraversalMode::Lint { .. } => Some(file_features.support_kind_for(FeatureKind::Lint)),
        //     TraversalMode::Migrate { .. } => None,
        //     TraversalMode::Search { .. } => {
        //         Some(file_features.support_kind_for(FeatureKind::Search))
        //     }
        // };

        if let Some(reason) = unsupported_reason {
            match reason {
                SupportKind::FileNotSupported => {
                    return Err(Message::from(
                        UnhandledDiagnostic.with_file_path(biome_path.to_string()),
                    ));
                }
                SupportKind::FeatureNotEnabled | SupportKind::Ignored => {
                    return Ok(FileStatus::Ignored);
                }
                SupportKind::Protected => {
                    return Ok(FileStatus::Protected(biome_path.to_string()));
                }
                SupportKind::Supported => {}
            };
        }

        Self::process_file(ctx, biome_path)
    }
}

impl ProcessFile for () {
    fn process_file<Ctx>(_ctx: &Ctx, _path: &BiomePath) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        Ok(FileStatus::Unchanged)
    }

    fn process_std_in(_payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic> {
        Ok(())
    }
}
