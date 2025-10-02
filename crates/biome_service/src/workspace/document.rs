use std::marker::PhantomData;
use biome_css_parser::CssOffsetParse;
use crate::diagnostics::FileTooLarge;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::Error;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::serde::Diagnostic as SerdeDiagnostic;
use biome_js_syntax::JsLanguage;
use biome_json_parser::JsonOffsetParse;
use biome_json_syntax::JsonLanguage;
use biome_parser::AnyParse;
use biome_rowan::{SyntaxNodeWithOffset, TextRange, TextSize};

#[derive(Debug, Clone)]
pub enum EmbeddedContent {
    Js(EmbeddedJsContent),
    Css(EmbeddedCssContent),
}

impl From<EmbeddedJsContent> for EmbeddedContent {
    fn from(content: EmbeddedJsContent) -> Self {
        Self::Js(content)
    }
}

impl From<EmbeddedCssContent> for EmbeddedContent {
    fn from(content: EmbeddedCssContent) -> Self {
        Self::Css(content)
    }
}

impl EmbeddedContent {
    pub const fn is_js(&self) -> bool {
        matches!(self, Self::Js(..))
    }

    pub const fn is_css(&self) -> bool {
        matches!(self, Self::Css(..))
    }

    pub fn as_js_embedded_content(&self) -> Option<EmbeddedJsContent> {
        if let Self::Js(content) = self {
            Some(content.clone())
        } else {
            None
        }
    }

    pub fn as_css_embedded_content(&self) -> Option<EmbeddedCssContent> {
        if let Self::Css(content) = self {
            Some(content.clone())
        } else {
            None
        }
    }

    pub fn content_range(&self) -> TextRange {
        match self {
            Self::Js(node) => node.content_range,
            Self::Css(node) => node.content_range,
        }
    }

    pub fn parse(&self) -> AnyParse {
        match self {
            Self::Js(node) => node.parse.clone(),
            Self::Css(node) => node.parse.clone(),
        }
    }

    pub fn file_source_index(&self) -> usize {
        match self {
            Self::Js(node) => node.file_source_index,
            Self::Css(node) => node.file_source_index,
        }
    }

    pub fn set_file_source_index(&mut self, index: usize) {
        match self {
            Self::Js(node) => node.file_source_index = index,
            Self::Css(node) => node.file_source_index = index,
        }
    }
}

impl From<JsonOffsetParse> for SendEmbeddedParse {
    fn from(value: JsonOffsetParse) -> Self {
        Self {
            root: value.syntax().clone().as_embedded_send(),
            diagnostics: value.into_diagnostics(),
        }
    }
}

impl From<CssOffsetParse> for SendEmbeddedParse {
    fn from(value: CssOffsetParse) -> Self {
        Self {
            root: value.syntax().clone().as_embedded_send(),
            diagnostics: value.into_diagnostics(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct EmbeddedLanguageSnippets {
    /// Embedded JavaScript content found in HTML documents (script tags).
    ///
    /// Each entry contains the parsed JavaScript with offset-aware positioning
    /// relative to the parent HTML document.
    pub scripts: Vec<EmbeddedContent<JsLanguage>>,

    /// Embedded JSON blocks found in HTML documents inside script tags.
    ///
    /// Each entry contains the parsed JSON with offset-aware positioning
    /// relative to the parent HTML document.
    pub json: Vec<EmbeddedContent<JsonLanguage>>,

    /// Embedded CSS content found in HTML documents (style tags).
    ///
    /// Each entry contains the parsed CSS with offset-aware positioning
    /// relative to the parent HTML document
    pub styles: Vec<EmbeddedContent<CssLanguage>>,
}

impl EmbeddedLanguageSnippets {
    pub fn get_format_nodes(
        &self,
        get_file_source: impl Fn(usize) -> DocumentFileSource,
    ) -> Vec<FormatEmbedNode> {
        self.scripts
            .iter()
            .map(|node| FormatEmbedNode {
                range: node.content_range,
                source: get_file_source(node.file_source_index),
                node: node.parse.clone(),
            })
            .chain(self.json.iter().map(|node| FormatEmbedNode {
                range: node.content_range,
                source: get_file_source(node.file_source_index),
                node: node.parse.clone(),
            }))
            .chain(self.styles.iter().map(|node| FormatEmbedNode {
                range: node.content_range,
                source: get_file_source(node.file_source_index),
                node: node.parse.clone(),
            }))
            .collect()
    }
}

/// Represents embedded content extracted from HTML documents.
///
/// This struct stores parsing metadata and provides access to the parsed
/// content with offset-aware positioning to maintain correct source locations.
#[derive(Clone, Debug)]
pub struct EmbeddedContent<L: ServiceLanguage + 'static> {
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
}

impl EmbeddedJsContent {
    /// Returns a syntax node
    pub fn node(&self) -> SyntaxNodeWithOffset<JsLanguage> {
        self.parse
            .unwrap_as_embedded_syntax_node()
            .into_node::<JsLanguage>()
    }

    /// This function transforms diagnostics coming from the parser into serializable diagnostics
    pub fn into_diagnostics(self) -> Vec<SerdeDiagnostic> {
        self.parse
            .into_diagnostics()
            .into_iter()
            .map(|mut diagnostic| {
                diagnostic.set_location_offset(self.content_offset);
                SerdeDiagnostic::new(Error::from(diagnostic))
            })
            .collect::<Vec<_>>()
    }
}

impl EmbeddedCssContent {
    /// Returns a syntax node
    pub fn node(&self) -> SyntaxNodeWithOffset<CssLanguage> {
        self.parse
            .unwrap_as_embedded_syntax_node()
            .into_node::<CssLanguage>()
    }

    /// This function transforms diagnostics coming from the parser into serializable diagnostics
    pub fn into_diagnostics(self) -> Vec<SerdeDiagnostic> {
        self.parse
            .into_diagnostics()
            .into_iter()
            .map(|mut diagnostic| {
                diagnostic.set_location_offset(self.content_offset);
                SerdeDiagnostic::new(Error::from(diagnostic))
            })
            .collect::<Vec<_>>()
    }

/// Represents embedded CSS content extracted from HTML documents.
///
/// This struct stores parsing metadata and provides access to the parsed content
/// with offset-aware positioning to maintain correct source locations.
#[derive(Clone, Debug)]
pub struct EmbeddedCssContent {
    /// The CSS source code extracted from the style element.
    pub parse: AnyParse,

    /// The range of the entire style element in the HTML document,
    /// including the opening and closing tags.
    pub element_range: TextRange,

    /// The range of just the CSS content within the style element,
    /// excluding the style tags themselves.
    pub content_range: TextRange,

    /// The offset where the CSS content starts in the parent document.
    /// This is used for offset-aware parsing.
    pub content_offset: TextSize,

    /// The file source of the embedded document
    pub file_source_index: usize,
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

    /// Embedded nodes content found in any document.
    pub(crate) embedded_nodes: Vec<EmbeddedContent>,

    /// Embedded content for foreign language snippets.
    pub(crate) embedded_snippets: EmbeddedLanguageSnippets,
}
