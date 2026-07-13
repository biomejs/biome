pub(crate) mod services;

use crate::diagnostics::FileTooLarge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProjectDataState {
    /// Shared project data reflects the current document.
    Current,
    /// Shared project data still needs to be refreshed for this document.
    RefreshPending,
    /// Only document-local state reflects the current content.
    DocumentOnly,
}

#[derive(Clone, Debug)]
pub(crate) struct Document {
    /// Document content.
    ///
    /// The content of the file is only available if it belongs to the project. For example, we don't
    /// want to store the content of files coming from dependencies.
    pub(crate) content: String,

    /// The version of the document.
    ///
    /// A version is only specified when the document is opened by a client,
    /// typically through the LSP. Documents that are only opened by the scanner
    /// do not have a version.
    pub(crate) version: Option<i32>,

    /// Whether shared project data reflects this version or a scan snapshot.
    pub(crate) project_data_state: ProjectDataState,

    /// The index of where the original file source is saved.
    /// Use `WorkspaceServer#file_sources` to retrieve the file source that belongs to the document.
    pub(crate) file_source_index: usize,

    /// The result of the parser (syntax tree + diagnostics).
    /// Types explained:
    /// - `Option`: if the file can be read
    /// - `Result`: if the file is read, but the file is too large
    /// - `AnyParse`: the result of the parsed file
    pub(crate) syntax: Option<Result<(), FileTooLarge>>,
}
