pub(crate) mod services;

use crate::diagnostics::FileTooLarge;
use crate::file_handlers::FormatEmbedNode;
use crate::settings::ServiceLanguage;
use crate::workspace::DocumentFileSource;
use crate::workspace::document::services::embedded_bindings::EmbeddedExportedBindings;
use crate::workspace::document::services::embedded_value_references::EmbeddedValueReferences;
use biome_css_syntax::{AnyCssRoot, CssLanguage};
use biome_diagnostics::Error;
use biome_diagnostics::serde::Diagnostic as SerdeDiagnostic;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, SyntaxNodeWithOffset, TextRange, TextSize};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum AnyEmbeddedSnippet {
    Js(EmbeddedSnippet<JsLanguage>, DocumentServices),
    Css(EmbeddedSnippet<CssLanguage>, DocumentServices),
    Json(EmbeddedSnippet<JsonLanguage>, DocumentServices),
}

impl From<EmbeddedSnippet<JsLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<JsLanguage>) -> Self {
        Self::Js(content, DocumentServices::none())
    }
}

impl From<(EmbeddedSnippet<CssLanguage>, DocumentServices)> for AnyEmbeddedSnippet {
    fn from(content: (EmbeddedSnippet<CssLanguage>, DocumentServices)) -> Self {
        Self::Css(content.0, content.1)
    }
}

impl From<EmbeddedSnippet<JsonLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<JsonLanguage>) -> Self {
        Self::Json(content, DocumentServices::none())
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
        if let Self::Js(content, _) = self {
            Some(content)
        } else {
            None
        }
    }

    pub fn as_css_embedded_snippet(&self) -> Option<&EmbeddedSnippet<CssLanguage>> {
        if let Self::Css(content, _) = self {
            Some(content)
        } else {
            None
        }
    }

    pub fn as_json_embedded_snippet(&self) -> Option<&EmbeddedSnippet<JsonLanguage>> {
        if let Self::Json(content, _) = self {
            Some(content)
        } else {
            None
        }
    }

    pub fn content_range(&self) -> TextRange {
        match self {
            Self::Js(node, _) => node.content_range,
            Self::Css(node, _) => node.content_range,
            Self::Json(node, _) => node.content_range,
        }
    }

    pub fn element_range(&self) -> TextRange {
        match self {
            Self::Js(node, _) => node.element_range,
            Self::Css(node, _) => node.element_range,
            Self::Json(node, _) => node.element_range,
        }
    }

    pub fn parse(&self) -> AnyParse {
        match self {
            Self::Js(node, _) => node.parse.clone(),
            Self::Css(node, _) => node.parse.clone(),
            Self::Json(node, _) => node.parse.clone(),
        }
    }

    pub fn file_source_index(&self) -> usize {
        match self {
            Self::Js(node, _) => node.file_source_index,
            Self::Css(node, _) => node.file_source_index,
            Self::Json(node, _) => node.file_source_index,
        }
    }

    pub fn set_file_source_index(&mut self, index: usize) {
        match self {
            Self::Js(node, _) => node.file_source_index = index,
            Self::Css(node, _) => node.file_source_index = index,
            Self::Json(node, _) => node.file_source_index = index,
        }
    }

    pub fn content_offset(&self) -> TextSize {
        match self {
            Self::Js(node, _) => node.content_offset,
            Self::Css(node, _) => node.content_offset,
            Self::Json(node, _) => node.content_offset,
        }
    }

    pub fn into_serde_diagnostics(self) -> Vec<SerdeDiagnostic> {
        match self {
            Self::Js(node, _) => node.into_serde_diagnostics(),
            Self::Css(node, _) => node.into_serde_diagnostics(),
            Self::Json(node, _) => node.into_serde_diagnostics(),
        }
    }

    pub fn as_snippet_services(&self) -> &DocumentServices {
        match self {
            Self::Js(_, services) => services,
            Self::Css(_, services) => services,
            Self::Json(_, services) => services,
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

    pub fn tree<N>(&self) -> N
    where
        N: AstNode,
        N::Language: 'static,
    {
        self.parse.tree()
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

    pub(crate) services: DocumentServices,
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

/// Represents the services that can be part of a document or a snippet.
/// These services must implement [Clone]. Also, they must support [Sync] and [Send]
///
/// To be sharable across threads, and a service needs to save language nodes, they must be
/// stored with [biome_rowan::AstPtr]. The service needs to accept the language root so
/// the pointer can be retrieved with its typed counter part.
#[derive(Clone, Debug)]
pub struct DocumentServices {
    /// Service to track bindings exported by the document
    exported_bindings: Option<EmbeddedExportedBindings>,

    /// Service to track value references from non-source snippets
    value_references: Option<EmbeddedValueReferences>,

    /// The document doesn't have any services
    language: LanguageServices,
}

impl DocumentServices {
    pub fn none() -> Self {
        Self {
            exported_bindings: None,
            value_references: None,
            language: LanguageServices::None,
        }
    }

    pub(crate) fn set_embedded_bindings(&mut self, bindings: EmbeddedExportedBindings) {
        self.exported_bindings = Some(bindings);
    }

    pub(crate) fn set_embedded_value_references(&mut self, value_refs: EmbeddedValueReferences) {
        self.value_references = Some(value_refs);
    }

    pub fn as_css_services(&self) -> Option<&CssDocumentServices> {
        if let LanguageServices::Css(services) = &self.language {
            Some(services)
        } else {
            None
        }
    }

    pub fn embedded_bindings(&self) -> Option<EmbeddedExportedBindings> {
        self.exported_bindings.clone()
    }

    pub fn embedded_value_references(&self) -> Option<EmbeddedValueReferences> {
        self.value_references.clone()
    }
}

#[derive(Clone, Debug)]
pub enum LanguageServices {
    /// The document doesn't have any services
    None,
    Css(CssDocumentServices),
}

impl From<CssDocumentServices> for DocumentServices {
    fn from(services: CssDocumentServices) -> Self {
        Self {
            exported_bindings: None,
            value_references: None,
            language: LanguageServices::Css(services),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct CssDocumentServices {
    /// Semantic model that belongs to the file
    pub(crate) semantic_model: Option<biome_css_semantic::model::SemanticModel>,
}

impl CssDocumentServices {
    pub fn with_css_semantic_model(mut self, root: &AnyCssRoot) -> Self {
        self.semantic_model = Some(biome_css_semantic::semantic_model(root));
        self
    }
}
