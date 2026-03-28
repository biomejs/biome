pub(crate) mod services;

use crate::diagnostics::FileTooLarge;
use crate::embed::languages::EmbeddedLanguageId;
use crate::file_handlers::FormatEmbedNode;
use crate::workspace::DocumentFileSource;
use crate::workspace::document::services::embedded_bindings::EmbeddedExportedBindings;
use crate::workspace::document::services::embedded_value_references::EmbeddedValueReferences;
use biome_css_syntax::{AnyCssRoot, CssLanguage};
use biome_diagnostics::Error;
use biome_diagnostics::serde::Diagnostic as SerdeDiagnostic;
use biome_graphql_syntax::GraphqlLanguage;
use biome_js_semantic::SemanticModelOptions;
use biome_js_syntax::{AnyJsRoot, JsLanguage};
use biome_json_syntax::JsonLanguage;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, SyntaxNodeWithOffset, TextRange, TextSize};
use biome_tailwind_syntax::TailwindLanguage;
use std::marker::PhantomData;

/// Marks syntax languages that are allowed to be stored as embedded snippets.
///
/// The associated `ID` provides the erased runtime identity used by workspace code
/// to dispatch snippet operations without depending on `DocumentFileSource`.
pub trait EmbeddedSnippetLanguage: biome_rowan::Language + 'static {
    const ID: EmbeddedLanguageId;
}

impl EmbeddedSnippetLanguage for JsLanguage {
    const ID: EmbeddedLanguageId = EmbeddedLanguageId::Js;
}

impl EmbeddedSnippetLanguage for CssLanguage {
    const ID: EmbeddedLanguageId = EmbeddedLanguageId::Css;
}

impl EmbeddedSnippetLanguage for JsonLanguage {
    const ID: EmbeddedLanguageId = EmbeddedLanguageId::Json;
}

impl EmbeddedSnippetLanguage for GraphqlLanguage {
    const ID: EmbeddedLanguageId = EmbeddedLanguageId::Graphql;
}

impl EmbeddedSnippetLanguage for TailwindLanguage {
    const ID: EmbeddedLanguageId = EmbeddedLanguageId::Tailwind;
}

/// Type-erased representation of an embedded snippet stored on a document.
///
/// This is the generic runtime form used by the workspace once a snippet has been
/// extracted and parsed. It carries the snippet language id, parse, source ranges,
/// and any snippet-scoped semantic services.
#[derive(Debug, Clone)]
pub struct AnyEmbeddedSnippet {
    embedded_language: EmbeddedLanguageId,
    file_source: Option<DocumentFileSource>,
    parse: AnyParse,
    element_range: TextRange,
    content_range: TextRange,
    content_offset: TextSize,
    services: DocumentServices,
}

impl From<(EmbeddedSnippet<JsLanguage>, DocumentServices)> for AnyEmbeddedSnippet {
    fn from(content: (EmbeddedSnippet<JsLanguage>, DocumentServices)) -> Self {
        Self::new(content.0, content.1)
    }
}

impl From<EmbeddedSnippet<JsLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<JsLanguage>) -> Self {
        Self::new(content, DocumentServices::none())
    }
}

impl From<(EmbeddedSnippet<CssLanguage>, DocumentServices)> for AnyEmbeddedSnippet {
    fn from(content: (EmbeddedSnippet<CssLanguage>, DocumentServices)) -> Self {
        Self::new(content.0, content.1)
    }
}

impl From<EmbeddedSnippet<CssLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<CssLanguage>) -> Self {
        Self::new(content, DocumentServices::none())
    }
}

impl From<EmbeddedSnippet<JsonLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<JsonLanguage>) -> Self {
        Self::new(content, DocumentServices::none())
    }
}

impl From<EmbeddedSnippet<GraphqlLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<GraphqlLanguage>) -> Self {
        Self::new(content, DocumentServices::none())
    }
}

impl From<(EmbeddedSnippet<TailwindLanguage>, DocumentServices)> for AnyEmbeddedSnippet {
    fn from(content: (EmbeddedSnippet<TailwindLanguage>, DocumentServices)) -> Self {
        Self::new(content.0, content.1)
    }
}

impl From<EmbeddedSnippet<TailwindLanguage>> for AnyEmbeddedSnippet {
    fn from(content: EmbeddedSnippet<TailwindLanguage>) -> Self {
        Self::new(content, DocumentServices::none())
    }
}

impl AnyEmbeddedSnippet {
    fn new<L: EmbeddedSnippetLanguage>(
        content: EmbeddedSnippet<L>,
        services: DocumentServices,
    ) -> Self {
        Self {
            embedded_language: L::ID,
            file_source: L::ID.document_file_source(),
            parse: content.parse,
            element_range: content.element_range,
            content_range: content.content_range,
            content_offset: content.content_offset,
            services,
        }
    }

    /// Returns the embedded language identity used for snippet dispatch.
    pub(crate) const fn language(&self) -> EmbeddedLanguageId {
        self.embedded_language
    }

    pub(crate) const fn file_source(&self) -> Option<DocumentFileSource> {
        self.file_source
    }

    pub(crate) fn with_file_source(mut self, file_source: DocumentFileSource) -> Self {
        self.file_source = Some(file_source);
        self
    }

    /// Returns the source range of the snippet contents inside the host document.
    pub fn content_range(&self) -> TextRange {
        self.content_range
    }

    /// Returns the source range of the full embed site inside the host document.
    pub fn element_range(&self) -> TextRange {
        self.element_range
    }

    /// Returns the parsed representation of the embedded snippet.
    pub fn parse(&self) -> AnyParse {
        self.parse.clone()
    }

    /// Returns the offset used to translate snippet diagnostics back to the host document.
    pub fn content_offset(&self) -> TextSize {
        self.content_offset
    }

    /// Converts parser diagnostics for this snippet into host-document-relative diagnostics.
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

    /// Returns the snippet-local services collected while processing this embed.
    pub fn as_snippet_services(&self) -> &DocumentServices {
        &self.services
    }
}

/// Typed representation of an extracted embedded snippet before type erasure.
///
/// This struct keeps the concrete syntax language for the snippet so parser results,
/// tree access, and root replacement can remain strongly typed until the snippet is
/// stored as `AnyEmbeddedSnippet`.
#[derive(Clone, Debug)]
pub struct EmbeddedSnippet<L: EmbeddedSnippetLanguage> {
    /// The parsed syntax tree for the snippet contents.
    pub parse: AnyParse,

    /// The range of the full embed site in the host document.
    pub element_range: TextRange,

    /// The range of the snippet contents within the host document.
    pub content_range: TextRange,

    /// The offset where the snippet contents start in the parent document.
    pub content_offset: TextSize,

    _phantom: PhantomData<L>,
}

impl<L: EmbeddedSnippetLanguage> EmbeddedSnippet<L> {
    /// Constructs a typed embedded snippet for a specific language.
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
            _phantom: PhantomData,
        }
    }

    /// Returns the embedded snippet as a syntax node with its original source offset.
    pub fn node(&self) -> SyntaxNodeWithOffset<L> {
        self.parse.unwrap_as_embedded_syntax_node().into_node::<L>()
    }

    /// Returns the typed AST root for this snippet.
    pub fn tree<N>(&self) -> N
    where
        N: AstNode,
        N::Language: 'static,
    {
        self.parse.tree()
    }

    /// Converts parser diagnostics for this snippet into host-document-relative diagnostics.
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
    pub fn get_embedded_snippets_format_nodes(&self) -> Vec<FormatEmbedNode> {
        self.embedded_snippets
            .iter()
            .filter_map(|node| {
                Some(FormatEmbedNode {
                    range: node.content_range(),
                    source: node.file_source()?,
                    node: node.parse().clone(),
                })
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

    pub fn as_js_services(&self) -> Option<&JsDocumentServices> {
        if let LanguageServices::Js(services) = &self.language {
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
    Js(JsDocumentServices),
    Css(CssDocumentServices),
}

impl From<JsDocumentServices> for DocumentServices {
    fn from(services: JsDocumentServices) -> Self {
        Self {
            exported_bindings: None,
            value_references: None,
            language: LanguageServices::Js(services),
        }
    }
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

#[derive(Clone, Default, Debug)]
pub struct JsDocumentServices {
    /// Semantic model that belongs to the file
    pub(crate) semantic_model: Option<biome_js_semantic::SemanticModel>,
}

impl JsDocumentServices {
    pub fn with_js_semantic_model(mut self, root: &AnyJsRoot) -> Self {
        self.semantic_model = Some(biome_js_semantic::semantic_model(
            root,
            SemanticModelOptions::default(),
        ));
        self
    }
}
