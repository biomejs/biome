use crate::runner::crawler::CrawlerContext;
use crate::runner::execution::Execution;
use biome_diagnostics::Error;
use biome_diagnostics::serde::Diagnostic;
use biome_fs::{BiomePath, FileSystem};
use biome_service::Workspace;
use biome_service::projects::ProjectKey;
use biome_service::workspace::CodeAction;

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

pub(crate) trait ProcessFile {
    fn process_file<Ctx>(ctx: &Ctx, path: BiomePath) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext;
}
