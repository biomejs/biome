use biome_parser::AnyParse;

use crate::diagnostics::FileTooLarge;

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

    /// The index of where the original file source is saved.
    /// Use `WorkspaceServer#file_sources` to retrieve the file source that belongs to the document.
    pub(crate) file_source_index: usize,

    /// The result of the parser (syntax tree + diagnostics).
    /// Types explained:
    /// - `Option`: if the file can be read
    /// - `Result`: if the file is read, but the  file is too large
    /// - `AnyParse`: the result of the parsed file
    pub(crate) syntax: Option<Result<AnyParse, FileTooLarge>>,

    /// If `true`, this indicates the document has been opened by the scanner,
    /// and should be unloaded only when the project is unregistered.
    ///
    /// Note the file can still *also* be opened explicitly by a client such as
    /// the LSP Proxy. In that case `version` will be `Some` and
    /// `opened_by_scanner` will be `true`, and the document will only be
    /// unloaded when both are unset.
    pub(super) opened_by_scanner: bool,
}
