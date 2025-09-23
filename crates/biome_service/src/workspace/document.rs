use std::marker::PhantomData;

use biome_css_parser::CssOffsetParse;
use biome_css_syntax::CssLanguage;
use biome_js_parser::JsOffsetParse;
use biome_js_syntax::JsLanguage;
use biome_json_parser::JsonOffsetParse;
use biome_json_syntax::JsonLanguage;
use biome_parser::AnyParse;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{EmbeddedSendNode, SyntaxNodeWithOffset, TextRange, TextSize};

use crate::{
    diagnostics::FileTooLarge, file_handlers::FormatEmbedNode, settings::ServiceLanguage,
    workspace::DocumentFileSource,
};

#[derive(Clone, Debug)]
pub struct SendEmbeddedParse {
    pub(crate) root: EmbeddedSendNode,
    #[expect(unused)]
    pub(crate) diagnostics: Vec<ParseDiagnostic>,
}

impl From<JsOffsetParse> for SendEmbeddedParse {
    fn from(value: JsOffsetParse) -> Self {
        Self {
            root: value.syntax().clone().as_embedded_send(),
            diagnostics: value.into_diagnostics(),
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
    pub parse: SendEmbeddedParse,

    /// The range of the entire script element in the HTML document,
    /// including the opening and closing tags.
    pub element_range: TextRange,

    /// The range of just the JavaScript content within the script element,
    /// excluding the script tags themselves.
    pub content_range: TextRange,

    /// The offset where the JavaScript content starts in the parent document.
    /// This is used for offset-aware parsing.
    pub content_offset: TextSize,

    /// The file source of the document.
    pub file_source_index: usize,

    _phantom: PhantomData<L>,
}

impl<L: ServiceLanguage + 'static> EmbeddedContent<L> {
    /// Constructs new embedded content for a specific language.
    pub fn new(
        parse: SendEmbeddedParse,
        element_range: TextRange,
        content_range: TextRange,
        content_offset: TextSize,
        file_source_index: usize,
    ) -> Self {
        Self {
            parse,
            element_range,
            content_range,
            content_offset,
            file_source_index,
            _phantom: PhantomData,
        }
    }

    /// Returns a syntax node.
    pub fn node(&self) -> SyntaxNodeWithOffset<L> {
        self.parse.root.clone().into_node::<L>()
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
    pub(crate) embedded_snippets: EmbeddedLanguageSnippets,
}
