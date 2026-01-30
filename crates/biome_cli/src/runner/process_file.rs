use crate::CliDiagnostic;
use crate::cli_options::CliOptions;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{ResultExt, ResultIoExt, UnhandledDiagnostic};
use crate::runner::execution::Execution;
use biome_console::Console;
use biome_diagnostics::{DiagnosticExt, DiagnosticTags, Error, category};
use biome_fs::{BiomePath, File, OpenOptions};
use biome_service::diagnostics::FileTooLarge;
use biome_service::file_handlers::DocumentFileSource;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    FeaturesSupported, FileExitsParams, FileFeaturesResult, SupportKind, SupportsFeatureParams,
};
use biome_service::workspace::{FileContent, FileGuard, OpenFileParams};
use biome_service::{Workspace, WorkspaceError};

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

impl FileStatus {
    pub const fn is_changed(&self) -> bool {
        matches!(self, Self::Changed)
    }
}

#[derive(Debug)]
pub(crate) enum MessageStat {
    Changed,
    Unchanged,
    Matches,
    Skipped,
}

/// Wrapper type for messages that can be printed during the traversal process
#[derive(Debug)]
pub(crate) enum Message {
    Stats(MessageStat),
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
    // DiagnosticsWithActions {
    //     file_path: String,
    //     content: String,
    //     diagnostics_with_actions: Vec<(Diagnostic, Vec<CodeAction>)>,
    //     skipped_diagnostics: u32,
    // },
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
    pub(crate) project_key: ProjectKey,
    pub(crate) workspace: &'a dyn Workspace,
    pub(crate) console: &'a mut dyn Console,
    pub(crate) cli_options: &'a CliOptions,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) skip_ignore_check: bool,
}

pub(crate) trait ProcessFile: Send + Sync + std::panic::RefUnwindSafe {
    fn process_file<Ctx>(
        ctx: &Ctx,
        workspace_file: &mut WorkspaceFile,
        features_supported: &FeaturesSupported,
    ) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext;

    fn process_std_in(payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic>;

    fn execute<Ctx>(ctx: &Ctx, biome_path: &BiomePath) -> FileResult
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
                features: ctx.execution().wanted_features(),
                inline_config: None,
                skip_ignore_check: false,
                not_requested_features: ctx.execution().not_requested_features(),
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

        if let Some(reason) = unsupported_reason {
            match reason {
                SupportKind::FileNotSupported => {
                    return Err(Message::from(
                        UnhandledDiagnostic.with_file_path(biome_path.to_string()),
                    ));
                }
                SupportKind::FeatureNotEnabled
                | SupportKind::Ignored
                | SupportKind::NotRequested => {
                    return Ok(FileStatus::Ignored);
                }
                SupportKind::Protected => {
                    return Ok(FileStatus::Protected(biome_path.to_string()));
                }
                SupportKind::Supported => {}
            };
        }

        let mut workspace_file = WorkspaceFile::new(ctx, biome_path.clone())?;
        let result = workspace_file.guard().check_file_size()?;
        if result.is_too_large() {
            ctx.push_diagnostic(
                FileTooLarge::from(result)
                    .with_file_path(workspace_file.path.to_string())
                    .with_category(ctx.execution().as_diagnostic_category()),
            );
            return Ok(FileStatus::Ignored);
        }

        Self::process_file(ctx, &mut workspace_file, &file_features)
    }

    fn should_skip_ignore_check(biome_path: &BiomePath, workspace: &dyn Workspace) -> bool {
        !workspace.fs().path_exists(biome_path.as_path())
    }
}

impl ProcessFile for () {
    fn process_file<Ctx>(
        _: &Ctx,
        _: &mut WorkspaceFile,
        _: &FeaturesSupported,
    ) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        Ok(FileStatus::Unchanged)
    }

    fn process_std_in(_payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic> {
        Ok(())
    }
}

/// Small wrapper that holds information and operations around the current processed file
pub(crate) struct WorkspaceFile<'ctx, 'app> {
    guard: FileGuard<'app, dyn Workspace + 'ctx>,
    file: Box<dyn File>,
    pub(crate) path: BiomePath,
}

impl<'ctx, 'app> WorkspaceFile<'ctx, 'app> {
    /// It attempts to read the file from disk, creating a [FileGuard] and
    /// saving these information internally
    pub(crate) fn new<Ctx>(ctx: &'ctx Ctx, path: BiomePath) -> Result<Self, Error>
    where
        Ctx: CrawlerContext,
    {
        let open_options = OpenOptions::default()
            .read(true)
            .write(ctx.execution().requires_write_access());

        let mut file = ctx
            .fs()
            .open_with_options(path.as_path(), open_options)
            .with_file_path(path.to_string())?;

        let mut input = String::new();
        file.read_to_string(&mut input)
            .with_file_path(path.to_string())?;

        let guard = FileGuard::new(ctx.workspace(), ctx.project_key(), path.clone())
            .with_file_path_and_code(path.to_string(), category!("internalError/fs"))?;

        if ctx.workspace().file_exists(FileExitsParams {
            file_path: path.clone(),
        })? {
            Ok(Self { guard, path, file })
        } else {
            let mut input = String::new();
            file.read_to_string(&mut input)
                .with_file_path(path.to_string())?;

            ctx.workspace().open_file(OpenFileParams {
                project_key: ctx.project_key(),
                document_file_source: None,
                path: path.clone(),
                content: FileContent::from_client(&input),
                persist_node_cache: false,
                inline_config: None,
            })?;

            Ok(Self { guard, path, file })
        }
    }

    pub(crate) fn guard(&self) -> &FileGuard<'app, dyn Workspace + 'ctx> {
        &self.guard
    }

    pub(crate) fn input(&self) -> Result<String, WorkspaceError> {
        self.guard().get_file_content()
    }

    pub(crate) fn as_extension(&self) -> Option<&str> {
        self.path.extension()
    }

    /// It updates the workspace file with `new_content`
    pub(crate) fn update_file(&mut self, new_content: impl Into<String>) -> Result<(), Error> {
        let new_content = new_content.into();

        self.file
            .set_content(new_content.as_bytes())
            .with_file_path(self.path.to_string())?;
        self.guard
            .change_file(self.file.file_version(), new_content)?;
        Ok(())
    }
}
