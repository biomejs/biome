use crate::diagnostics::FileTooLarge;
use crate::file_handlers::FormatEmbedNode;
use crate::settings::ServiceLanguage;
use crate::workspace::DocumentFileSource;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::Error;
use biome_diagnostics::serde::Diagnostic as SerdeDiagnostic;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_parser::AnyParse;
use biome_rowan::{SyntaxNodeWithOffset, TextRange, TextSize};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum AnyEmbeddedSnippet {
    Js(EmbeddedSnippet<JsLanguage>),
    Css(EmbeddedSnippet<CssLanguage>),
    Json(EmbeddedSnippet<JsonLanguage>),
}

impl From<EmbeddedSnippet<JsLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<JsLanguage>) -> Self {
        Self::Js(content)
    }
}

impl From<EmbeddedSnippet<CssLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<CssLanguage>) -> Self {
        Self::Css(content)
    }
}

impl From<EmbeddedSnippet<JsonLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<JsonLanguage>) -> Self {
        Self::Json(content)
    }
}

impl AnyEmbeddedSnippet {
    pub const fn is_js(&self) -> bool {
        matches!(self, Self::Js(..))
    }

    pub const fn is_css(&self) -> bool {
        matches!(self, Self::Css(..))
    }

    pub fn as_js_embedded_snippet(&self) -> Option<&EmbeddedSnippet<JsLanguage>> {
        if let Self::Js(content) = self {
            Some(content)
        } else {
            None
        }
    }

    pub fn as_css_embedded_snippet(&self) -> Option<&EmbeddedSnippet<CssLanguage>> {
        if let Self::Css(content) = self {
            Some(content)
        } else {
            None
        }
    }

    pub fn as_json_embedded_snippet(&self) -> Option<&EmbeddedSnippet<JsonLanguage>> {
        if let Self::Json(content) = self {
            Some(content)
        } else {
            None
        }
    }

    pub fn content_range(&self) -> TextRange {
        match self {
            Self::Js(node) => node.content_range,
            Self::Css(node) => node.content_range,
            Self::Json(node) => node.content_range,
        }
    }

    pub fn element_range(&self) -> TextRange {
        match self {
            Self::Js(node) => node.element_range,
            Self::Css(node) => node.element_range,
            Self::Json(node) => node.element_range,
        }
    }

    pub fn parse(&self) -> AnyParse {
        match self {
            Self::Js(node) => node.parse.clone(),
            Self::Css(node) => node.parse.clone(),
            Self::Json(node) => node.parse.clone(),
        }
    }

    pub fn file_source_index(&self) -> usize {
        match self {
            Self::Js(node) => node.file_source_index,
            Self::Css(node) => node.file_source_index,
            Self::Json(node) => node.file_source_index,
        }
    }

    pub fn set_file_source_index(&mut self, index: usize) {
        match self {
            Self::Js(node) => node.file_source_index = index,
            Self::Css(node) => node.file_source_index = index,
            Self::Json(node) => node.file_source_index = index,
        }
    }

    pub fn content_offset(&self) -> TextSize {
        match self {
            Self::Js(node) => node.content_offset,
            Self::Css(node) => node.content_offset,
            Self::Json(node) => node.content_offset,
        }
    }

    pub fn into_serde_diagnostics(self) -> Vec<SerdeDiagnostic> {
        match self {
            Self::Js(node) => node.into_serde_diagnostics(),
            Self::Css(node) => node.into_serde_diagnostics(),
            Self::Json(node) => node.into_serde_diagnostics(),
        }
    }
}

/// Represents embedded content extracted from HTML documents.
///
/// This struct stores parsing metadata and provides access to the parsed
/// content with offset-aware positioning to maintain correct source locations.
#[derive(Clone, Debug)]
pub struct EmbeddedSnippet<L: ServiceLanguage + 'static> {
    /// The JavaScript source code extracted from the script element.
    pub parse: AnyParse,

    /// The range of the entire script element in the HTML document,
    /// including the opening and closing tags.
    pub element_range: TextRange,

    /// The range of just the JavaScript content within the script element,
    /// excluding the script tags themselves.
    pub content_range: TextRange,

    /// The offset where the JavaScript content starts in the parent document.
    /// This is used for offset-aware parsing.
    pub content_offset: TextSize,

    /// The file source of the document
    pub file_source_index: usize,

    _phantom: PhantomData<L>,
}

impl<L: ServiceLanguage + 'static> EmbeddedSnippet<L> {
    /// Constructs new embedded content for a specific language.
    pub fn new(
        parse: AnyParse,
        element_range: TextRange,
        content_range: TextRange,
        content_offset: TextSize,
    ) -> Self {
        Self {
            parse,
            element_range,
            content_range,
            content_offset,
            file_source_index: Default::default(),
            _phantom: PhantomData,
        }
    }

    /// Returns a syntax node.
    pub fn node(&self) -> SyntaxNodeWithOffset<L> {
        self.parse.unwrap_as_embedded_syntax_node().into_node::<L>()
    }

    /// This function transforms diagnostics coming from the parser into serializable diagnostics
    pub fn into_serde_diagnostics(self) -> Vec<SerdeDiagnostic> {
        self.parse
            .into_diagnostics()
            .into_iter()
            .map(|mut diagnostic| {
                diagnostic.set_location_offset(self.content_offset);
                SerdeDiagnostic::new(Error::from(diagnostic))
            })
            .collect::<Vec<_>>()
    }

    pub fn set_file_source_index(&mut self, index: usize) {
        self.file_source_index = index;
    }
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
    /// - `Result`: if the file is read, but the file is too large
    /// - `AnyParse`: the result of the parsed file
    pub(crate) syntax: Option<Result<AnyParse, FileTooLarge>>,

    /// Embedded content for foreign language snippets.
    pub(crate) embedded_snippets: Vec<AnyEmbeddedSnippet>,
}

impl Document {
    pub fn get_embedded_snippets_format_nodes(
        &self,
        get_file_source: impl Fn(usize) -> DocumentFileSource,
    ) -> Vec<FormatEmbedNode> {
        self.embedded_snippets
            .iter()
            .map(|node| FormatEmbedNode {
                range: node.content_range(),
                source: get_file_source(node.file_source_index()),
                node: node.parse().clone(),
            })
            .collect()
    }
}
