use biome_css_parser::CssOffsetParse;
use biome_js_parser::JsOffsetParse;
use biome_js_syntax::JsLanguage;
use biome_parser::AnyParse;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{EmbeddedSendNode, SyntaxNodeWithOffset, TextRange, TextSize};

use crate::diagnostics::FileTooLarge;

#[derive(Clone, Debug)]
pub struct SendJsEmbeddedParse {
    pub(crate) root: EmbeddedSendNode,
    #[expect(unused)]
    pub(crate) diagnostics: Vec<ParseDiagnostic>,
}

impl From<JsOffsetParse> for SendJsEmbeddedParse {
    fn from(value: JsOffsetParse) -> Self {
        Self {
            root: value.syntax().clone().as_embedded_send(),
            diagnostics: value.into_diagnostics(),
        }
    }
}

/// Represents embedded JavaScript content extracted from HTML documents.
///
/// This struct stores parsing metadata and provides access to the parsed content
/// with offset-aware positioning to maintain correct source locations.
#[derive(Clone, Debug)]
pub struct EmbeddedJsContent {
    /// The JavaScript source code extracted from the script element.
    pub parse: SendJsEmbeddedParse,

    /// The range of the entire script element in the HTML document,
    /// including the opening and closing tags.
    pub element_range: TextRange,

    /// The range of just the JavaScript content within the script element,
    /// excluding the script tags themselves.
    pub content_range: TextRange,

    /// The offset where the JavaScript content starts in the parent document.
    /// This is used for offset-aware parsing.
    pub content_offset: TextSize,
}

impl EmbeddedJsContent {
    /// Returns a syntax node
    pub fn node(&self) -> SyntaxNodeWithOffset<JsLanguage> {
        self.parse.root.clone().into_node::<JsLanguage>()
    }
}

#[derive(Clone, Debug)]
pub struct SendCssEmbeddedParse {
    pub(crate) root: EmbeddedSendNode,
    #[expect(unused)]
    pub(crate) diagnostics: Vec<ParseDiagnostic>,
}

impl EmbeddedCssContent {
    /// Returns a syntax node
    pub fn node(&self) -> SyntaxNodeWithOffset<JsLanguage> {
        self.parse.root.clone().into_node::<JsLanguage>()
    }
}

impl From<CssOffsetParse> for SendCssEmbeddedParse {
    fn from(value: CssOffsetParse) -> Self {
        Self {
            root: value.syntax().clone().as_embedded_send(),
            diagnostics: value.into_diagnostics(),
        }
    }
}

/// Represents embedded CSS content extracted from HTML documents.
///
/// This struct stores parsing metadata and provides access to the parsed content
/// with offset-aware positioning to maintain correct source locations.
#[derive(Clone, Debug)]
pub struct EmbeddedCssContent {
    /// The CSS source code extracted from the style element.
    pub parse: SendCssEmbeddedParse,

    /// The range of the entire style element in the HTML document,
    /// including the opening and closing tags.
    pub element_range: TextRange,

    /// The range of just the CSS content within the style element,
    /// excluding the style tags themselves.
    pub content_range: TextRange,

    /// The offset where the CSS content starts in the parent document.
    /// This is used for offset-aware parsing.
    pub content_offset: TextSize,
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

    /// Embedded JavaScript content found in HTML documents (script tags).
    /// Each entry contains the parsed JavaScript with offset-aware positioning
    /// relative to the parent HTML document.
    // FIXME: remove underscore once we start reading the field
    pub(crate) _embedded_scripts: Vec<EmbeddedJsContent>,

    /// Embedded CSS content found in HTML documents (style tags).
    /// Each entry contains the parsed CSS with offset-aware positioning
    /// relative to the parent HTML document
    // FIXME: remove underscore once we start reading the field
    pub(crate) _embedded_styles: Vec<EmbeddedCssContent>,
}
