use crate::WorkspaceError;
use crate::file_handlers::{
    Capabilities, CodeActionsParams, DocumentFileSource, Features, FixAllParams, LintParams,
    LintResults, tailwind,
};
use crate::workspace::{FixFileResult, PullActionsResult};
use biome_css_syntax::{CssFileSource, CssLanguage};
use biome_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_parser::AnyParse;
use biome_rowan::{SendNode, SyntaxNodeWithOffset};
use biome_tailwind_syntax::TailwindLanguage;

/// Identifies the language of an embedded snippet independently from the host file.
///
/// Unlike `DocumentFileSource`, this enum can represent languages that only exist as
/// extracted snippets and never as top-level files, such as Tailwind class lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddedLanguageId {
    Js,
    Css,
    Json,
    Graphql,
    Tailwind,
}

impl EmbeddedLanguageId {
    /// Returns the service capabilities used to analyze snippets of this embedded language.
    pub(crate) fn capabilities(
        self,
        features: &Features,
        file_source: DocumentFileSource,
    ) -> Option<Capabilities> {
        match self {
            Self::Tailwind => Some(tailwind::capabilities()),
            Self::Js | Self::Css | Self::Json | Self::Graphql => {
                Some(features.get_capabilities(file_source))
            }
        }
    }

    /// Maps an embedded language back to a top-level file source when one exists.
    ///
    /// Embedded-only languages return `None` so callers can avoid routing them through
    /// top-level file handling code.
    pub(crate) fn document_file_source(self) -> Option<DocumentFileSource> {
        match self {
            Self::Js => Some(DocumentFileSource::Js(JsFileSource::js_module())),
            Self::Css => Some(DocumentFileSource::Css(CssFileSource::css())),
            Self::Json => Some(DocumentFileSource::Json(JsonFileSource::json())),
            Self::Graphql => Some(DocumentFileSource::Graphql(GraphqlFileSource::graphql())),
            Self::Tailwind => None,
        }
    }

    /// Runs lint analysis for an embedded snippet of this language.
    pub(crate) fn lint(
        self,
        features: &Features,
        file_source: DocumentFileSource,
        params: LintParams,
    ) -> Option<LintResults> {
        match self {
            Self::Tailwind => Some(tailwind::lint(params)),
            Self::Js | Self::Css | Self::Json | Self::Graphql => features
                .get_capabilities(file_source)
                .analyzer
                .lint
                .map(|lint| lint(params)),
        }
    }

    /// Computes code actions for an embedded snippet of this language.
    pub(crate) fn code_actions(
        self,
        features: &Features,
        file_source: DocumentFileSource,
        params: CodeActionsParams,
    ) -> Option<PullActionsResult> {
        match self {
            Self::Tailwind => Some(tailwind::code_actions(params)),
            Self::Js | Self::Css | Self::Json | Self::Graphql => features
                .get_capabilities(file_source)
                .analyzer
                .code_actions
                .map(|code_actions| code_actions(params)),
        }
    }

    /// Applies all safe fixes for an embedded snippet of this language.
    pub(crate) fn fix_all(
        self,
        features: &Features,
        file_source: DocumentFileSource,
        params: FixAllParams,
    ) -> Option<Result<FixFileResult, WorkspaceError>> {
        match self {
            Self::Tailwind => Some(tailwind::fix_all(params)),
            Self::Js | Self::Css | Self::Json | Self::Graphql => features
                .get_capabilities(file_source)
                .analyzer
                .fix_all
                .map(|fix_all| fix_all(params)),
        }
    }

    /// Replaces the parsed root of an embedded snippet after applying fixes.
    ///
    /// The host document stores snippets in erased form, so this method restores the
    /// correct concrete syntax type before updating the embedded parse tree.
    pub(crate) fn set_new_root(
        self,
        parse: &mut AnyParse,
        new_root: SendNode,
        file_source: Option<DocumentFileSource>,
    ) {
        let _ = file_source;
        match self {
            Self::Js => {
                let node = new_root.into_node::<JsLanguage>().unwrap();
                let current = parse
                    .unwrap_as_embedded_syntax_node()
                    .into_node::<JsLanguage>();
                parse.set_new_embedded_root(
                    SyntaxNodeWithOffset::new(node, current.base_offset()).as_embedded_send(),
                );
            }
            Self::Css => {
                let node = new_root.into_node::<CssLanguage>().unwrap();
                let current = parse
                    .unwrap_as_embedded_syntax_node()
                    .into_node::<CssLanguage>();
                parse.set_new_embedded_root(
                    SyntaxNodeWithOffset::new(node, current.base_offset()).as_embedded_send(),
                );
            }
            Self::Json => {
                let node = new_root.into_node::<JsonLanguage>().unwrap();
                let current = parse
                    .unwrap_as_embedded_syntax_node()
                    .into_node::<JsonLanguage>();
                parse.set_new_embedded_root(
                    SyntaxNodeWithOffset::new(node, current.base_offset()).as_embedded_send(),
                );
            }
            Self::Graphql => {
                let node = new_root.into_node::<GraphqlLanguage>().unwrap();
                let current = parse
                    .unwrap_as_embedded_syntax_node()
                    .into_node::<GraphqlLanguage>();
                parse.set_new_embedded_root(
                    SyntaxNodeWithOffset::new(node, current.base_offset()).as_embedded_send(),
                );
            }
            Self::Tailwind => {
                let node = new_root.into_node::<TailwindLanguage>().unwrap();
                let current = parse
                    .unwrap_as_embedded_syntax_node()
                    .into_node::<TailwindLanguage>();
                parse.set_new_embedded_root(
                    SyntaxNodeWithOffset::new(node, current.base_offset()).as_embedded_send(),
                );
            }
        }
    }
}
