use self::{
    css::CssFileHandler, javascript::JsFileHandler, json::JsonFileHandler,
    unknown::UnknownFileHandler,
};
use crate::diagnostics::{QueryDiagnostic, SearchError};
pub use crate::file_handlers::astro::{AstroFileHandler, ASTRO_FENCE};
use crate::file_handlers::graphql::GraphqlFileHandler;
pub use crate::file_handlers::svelte::{SvelteFileHandler, SVELTE_FENCE};
pub use crate::file_handlers::vue::{VueFileHandler, VUE_FENCE};
use crate::settings::{Settings, WorkspaceSettingsHandle};
use crate::workspace::{
    FixFileMode, FixFileResult, GetSyntaxTreeResult, PullActionsResult, RenameResult,
};
use crate::WorkspaceError;
use biome_analyze::{
    AnalyzerDiagnostic, AnalyzerOptions, AnalyzerPluginVec, AnalyzerSignal, ControlFlow,
    GroupCategory, Never, Queryable, RegistryVisitor, Rule, RuleCategories, RuleCategory,
    RuleFilter, RuleGroup,
};
use biome_configuration::analyzer::{RuleDomainValue, RuleSelector};
use biome_configuration::Rules;
use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_css_analyze::METADATA as css_metadata;
use biome_css_syntax::{CssFileSource, CssLanguage};
use biome_dependency_graph::DependencyGraph;
use biome_diagnostics::{category, Diagnostic, DiagnosticExt, Severity};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_graphql_analyze::METADATA as graphql_metadata;
use biome_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use biome_grit_patterns::{GritQuery, GritQueryEffect, GritTargetFile};
use biome_grit_syntax::file_source::GritFileSource;
use biome_html_syntax::HtmlFileSource;
use biome_js_analyze::METADATA as js_metadata;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{
    EmbeddingKind, JsFileSource, JsLanguage, Language, LanguageVariant, TextRange, TextSize,
};
use biome_json_analyze::METADATA as json_metadata;
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_parser::AnyParse;
use biome_project_layout::ProjectLayout;
use biome_rowan::{FileSourceError, NodeCache};
use biome_string_case::StrLikeExtension;

use camino::Utf8Path;
use grit::GritFileHandler;
use html::HtmlFileHandler;
pub use javascript::JsFormatterSettings;
use rustc_hash::FxHashSet;
use std::borrow::Cow;
use std::sync::Arc;
use tracing::instrument;

mod astro;
pub(crate) mod css;
pub(crate) mod graphql;
pub(crate) mod grit;
pub(crate) mod html;
pub(crate) mod javascript;
pub(crate) mod json;
mod svelte;
mod unknown;
mod vue;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum DocumentFileSource {
    Js(JsFileSource),
    Json(JsonFileSource),
    Css(CssFileSource),
    Graphql(GraphqlFileSource),
    Html(HtmlFileSource),
    Grit(GritFileSource),
    #[default]
    Unknown,
}

impl From<JsFileSource> for DocumentFileSource {
    fn from(value: JsFileSource) -> Self {
        Self::Js(value)
    }
}

impl From<JsonFileSource> for DocumentFileSource {
    fn from(value: JsonFileSource) -> Self {
        Self::Json(value)
    }
}

impl From<CssFileSource> for DocumentFileSource {
    fn from(value: CssFileSource) -> Self {
        Self::Css(value)
    }
}

impl From<GraphqlFileSource> for DocumentFileSource {
    fn from(value: GraphqlFileSource) -> Self {
        Self::Graphql(value)
    }
}

impl From<HtmlFileSource> for DocumentFileSource {
    fn from(value: HtmlFileSource) -> Self {
        Self::Html(value)
    }
}

impl From<GritFileSource> for DocumentFileSource {
    fn from(value: GritFileSource) -> Self {
        Self::Grit(value)
    }
}

impl From<&Utf8Path> for DocumentFileSource {
    fn from(path: &Utf8Path) -> Self {
        Self::from_path(path)
    }
}

impl DocumentFileSource {
    #[instrument(level = "debug", fields(result))]
    fn try_from_well_known(path: &Utf8Path) -> Result<Self, FileSourceError> {
        if let Ok(file_source) = JsonFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = JsFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = CssFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = GraphqlFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }

        Err(FileSourceError::UnknownFileName)
    }

    /// Returns the document file source corresponding to this file name from well-known files
    pub fn from_well_known(path: &Utf8Path) -> Self {
        Self::try_from_well_known(path).unwrap_or(DocumentFileSource::Unknown)
    }

    #[instrument(level = "debug", fields(result))]
    fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        if let Ok(file_source) = JsonFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = JsFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = CssFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = GraphqlFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "experimental-html")]
        if let Ok(file_source) = HtmlFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = GritFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        Err(FileSourceError::UnknownExtension)
    }

    /// Returns the document file source corresponding to this file extension
    pub fn from_extension(extension: &str) -> Self {
        Self::try_from_extension(extension).unwrap_or(DocumentFileSource::Unknown)
    }

    #[instrument(level = "debug", fields(result))]
    fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        if let Ok(file_source) = JsonFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = JsFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = CssFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = GraphqlFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "experimental-html")]
        if let Ok(file_source) = HtmlFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = GritFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        Err(FileSourceError::UnknownLanguageId)
    }

    /// Returns the document file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn from_language_id(language_id: &str) -> Self {
        Self::try_from_language_id(language_id).unwrap_or(DocumentFileSource::Unknown)
    }

    #[instrument(level = "debug", fields(result))]
    pub(crate) fn try_from_path(path: &Utf8Path) -> Result<Self, FileSourceError> {
        if let Ok(file_source) = Self::try_from_well_known(path) {
            return Ok(file_source);
        }

        let filename = path
            .file_name()
            // We assume the file extensions are case-insensitive.
            // Thus, we normalize the filrname to lowercase.
            .map(|filename| filename.to_ascii_lowercase_cow());

        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        // TODO: This should be extracted to a dedicated function, maybe in biome_fs
        // because the same logic is also used in JsFileSource::try_from
        // and we may support more and more extensions with more than one dots.
        let extension = &match filename {
            Some(filename) if filename.ends_with(".d.ts") => Cow::Borrowed("d.ts"),
            Some(filename) if filename.ends_with(".d.mts") => Cow::Borrowed("d.mts"),
            Some(filename) if filename.ends_with(".d.cts") => Cow::Borrowed("d.cts"),
            _ => path
                .extension()
                // We assume the file extensions are case-insensitive.
                // Thus, we normalize the extension to lowercase.
                .map(|ext| ext.to_ascii_lowercase_cow())
                .ok_or(FileSourceError::MissingFileExtension)?,
        };

        Self::try_from_extension(extension.as_ref())
    }

    /// Returns the document file source corresponding to the file path
    pub fn from_path(path: &Utf8Path) -> Self {
        Self::try_from_path(path).unwrap_or(DocumentFileSource::Unknown)
    }

    /// Returns the document file source if it's not unknown, otherwise returns `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_syntax::JsFileSource;
    /// use biome_json_syntax::JsonFileSource;
    /// use biome_service::workspace::DocumentFileSource;
    /// let x = DocumentFileSource::from(JsFileSource::js_module());
    /// let y = DocumentFileSource::Unknown;
    /// assert_eq!(x.or(y), JsFileSource::js_module().into());
    ///
    /// let x = DocumentFileSource::Unknown;
    /// let y = DocumentFileSource::from(JsFileSource::js_module());
    /// assert_eq!(x.or(y), JsFileSource::js_module().into());
    ///
    /// let x = DocumentFileSource::from(JsFileSource::js_module());
    /// let y = DocumentFileSource::from(JsonFileSource::json());
    /// assert_eq!(x.or(y), JsFileSource::js_module().into());
    ///
    /// let x = DocumentFileSource::Unknown;
    /// let y = DocumentFileSource::Unknown;
    /// assert_eq!(x.or(y), DocumentFileSource::Unknown);
    /// ```
    pub fn or(self, other: DocumentFileSource) -> DocumentFileSource {
        if self != DocumentFileSource::Unknown {
            self
        } else {
            other
        }
    }

    pub const fn is_javascript_like(&self) -> bool {
        matches!(self, DocumentFileSource::Js(_))
    }

    pub const fn is_json_like(&self) -> bool {
        matches!(self, DocumentFileSource::Json(_))
    }

    pub const fn is_css_like(&self) -> bool {
        matches!(self, DocumentFileSource::Css(_))
    }

    pub fn to_js_file_source(&self) -> Option<JsFileSource> {
        match self {
            DocumentFileSource::Js(file_source) => Some(*file_source),
            _ => None,
        }
    }

    pub fn to_json_file_source(&self) -> Option<JsonFileSource> {
        match self {
            DocumentFileSource::Json(json) => Some(*json),
            _ => None,
        }
    }

    pub fn to_graphql_file_source(&self) -> Option<GraphqlFileSource> {
        match self {
            DocumentFileSource::Graphql(graphql) => Some(*graphql),
            _ => None,
        }
    }

    pub fn to_grit_file_source(&self) -> Option<GritFileSource> {
        match self {
            DocumentFileSource::Grit(grit) => Some(*grit),
            _ => None,
        }
    }

    pub fn to_css_file_source(&self) -> Option<CssFileSource> {
        match self {
            DocumentFileSource::Css(css) => Some(*css),
            _ => None,
        }
    }

    pub fn to_html_file_source(&self) -> Option<HtmlFileSource> {
        match self {
            DocumentFileSource::Html(html) => Some(*html),
            _ => None,
        }
    }

    pub fn can_parse(path: &Utf8Path, content: &str) -> bool {
        let file_source = DocumentFileSource::from(path);
        match file_source {
            DocumentFileSource::Js(js) => match js.as_embedding_kind() {
                EmbeddingKind::Astro => ASTRO_FENCE.is_match(content),
                EmbeddingKind::Vue => VUE_FENCE.is_match(content),
                EmbeddingKind::Svelte => SVELTE_FENCE.is_match(content),
                EmbeddingKind::None => true,
            },
            DocumentFileSource::Css(_)
            | DocumentFileSource::Graphql(_)
            | DocumentFileSource::Json(_)
            | DocumentFileSource::Grit(_) => true,
            DocumentFileSource::Html(_) => cfg!(feature = "experimental-html"),
            DocumentFileSource::Unknown => false,
        }
    }
}

impl biome_console::fmt::Display for DocumentFileSource {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            DocumentFileSource::Js(js) => {
                let is_jsx = js.is_jsx();
                if js.is_typescript() {
                    if is_jsx {
                        fmt.write_markup(markup! { "TSX" })
                    } else {
                        fmt.write_markup(markup! { "TypeScript" })
                    }
                } else if is_jsx {
                    fmt.write_markup(markup! { "JSX" })
                } else {
                    fmt.write_markup(markup! { "JavaScript" })
                }
            }
            DocumentFileSource::Json(json) => {
                if json.allow_comments() {
                    fmt.write_markup(markup! { "JSONC" })
                } else {
                    fmt.write_markup(markup! { "JSON" })
                }
            }
            DocumentFileSource::Css(_) => fmt.write_markup(markup! { "CSS" }),
            DocumentFileSource::Graphql(_) => fmt.write_markup(markup! { "GraphQL" }),
            DocumentFileSource::Html(_) => fmt.write_markup(markup! { "HTML" }),
            DocumentFileSource::Grit(_) => fmt.write_markup(markup! { "Grit" }),
            DocumentFileSource::Unknown => fmt.write_markup(markup! { "Unknown" }),
        }
    }
}

pub struct FixAllParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) workspace: WorkspaceSettingsHandle,
    /// Whether it should format the code action
    pub(crate) should_format: bool,
    pub(crate) biome_path: &'a BiomePath,
    pub(crate) dependency_graph: Arc<DependencyGraph>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) document_file_source: DocumentFileSource,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) rule_categories: RuleCategories,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: Vec<RuleSelector>,
    pub(crate) plugins: AnalyzerPluginVec,
}

#[derive(Default)]
/// The list of capabilities that are available for a language
pub struct Capabilities {
    pub(crate) parser: ParserCapabilities,
    pub(crate) debug: DebugCapabilities,
    pub(crate) analyzer: AnalyzerCapabilities,
    pub(crate) formatter: FormatterCapabilities,
    pub(crate) search: SearchCapabilities,
    pub(crate) enabled_for_path: EnabledForPath,
}

#[derive(Clone)]
pub struct ParseResult {
    pub(crate) any_parse: AnyParse,
    pub(crate) language: Option<DocumentFileSource>,
}

type Parse = fn(
    &BiomePath,
    DocumentFileSource,
    &str,
    WorkspaceSettingsHandle,
    &mut NodeCache,
) -> ParseResult;

#[derive(Default)]
pub struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,
}

type DebugSyntaxTree = fn(&BiomePath, AnyParse) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(AnyParse, TextSize) -> String;
type DebugFormatterIR = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    WorkspaceSettingsHandle,
) -> Result<String, WorkspaceError>;

#[derive(Default)]
pub struct DebugCapabilities {
    /// Prints the syntax tree
    pub(crate) debug_syntax_tree: Option<DebugSyntaxTree>,
    /// Prints the control flow graph
    pub(crate) debug_control_flow: Option<DebugControlFlow>,
    /// Prints the formatter IR
    pub(crate) debug_formatter_ir: Option<DebugFormatterIR>,
}

#[derive(Debug)]
pub(crate) struct LintParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) workspace: &'a WorkspaceSettingsHandle,
    pub(crate) language: DocumentFileSource,
    pub(crate) max_diagnostics: u32,
    pub(crate) path: &'a BiomePath,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) categories: RuleCategories,
    pub(crate) dependency_graph: Arc<DependencyGraph>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: Vec<RuleSelector>,
    pub(crate) plugins: AnalyzerPluginVec,
}

pub(crate) struct LintResults {
    pub(crate) diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    pub(crate) errors: usize,
    pub(crate) skipped_diagnostics: u32,
}

pub(crate) struct ProcessLint<'a> {
    diagnostic_count: u32,
    errors: usize,
    diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    ignores_suppression_comment: bool,
    rules: Option<Cow<'a, Rules>>,
    max_diagnostics: u32,
}

impl<'a> ProcessLint<'a> {
    pub(crate) fn new(params: &LintParams<'a>) -> Self {
        Self {
            diagnostic_count: params.parse.diagnostics().len() as u32,
            errors: Default::default(),
            diagnostics: Default::default(),
            // Do not report unused suppression comment diagnostics if:
            // - it is a syntax-only analyzer pass, or
            // - if a single rule is run.
            ignores_suppression_comment: !params.categories.contains(RuleCategory::Lint)
                || !params.only.is_empty(),
            rules: params
                .workspace
                .settings()
                .as_ref()
                .and_then(|settings| settings.as_linter_rules(params.path.as_path())),
            max_diagnostics: params.max_diagnostics,
        }
    }

    pub(crate) fn process_signal<L: biome_rowan::Language>(
        &mut self,
        signal: &dyn AnalyzerSignal<L>,
    ) -> ControlFlow {
        if let Some(mut diagnostic) = signal.diagnostic() {
            if self.ignores_suppression_comment
                && diagnostic.category() == Some(category!("suppressions/unused"))
            {
                return ControlFlow::<Never>::Continue(());
            }

            self.diagnostic_count += 1;

            // We do now check if the severity of the diagnostics should be changed.
            // The configuration allows to change the severity of the diagnostics emitted by rules.
            let severity = diagnostic
                .category()
                .filter(|category| category.name().starts_with("lint/"))
                .and_then(|category| {
                    self.rules.as_ref().and_then(|rules| {
                        rules.get_severity_from_category(category, diagnostic.severity())
                    })
                })
                .or_else(|| Some(diagnostic.severity()))
                .unwrap_or(Severity::Warning);

            if severity >= Severity::Error {
                self.errors += 1;
            }

            if self.diagnostic_count <= self.max_diagnostics {
                for action in signal.actions() {
                    if !action.is_suppression() {
                        diagnostic = diagnostic.add_code_suggestion(action.into());
                    }
                }

                let error = diagnostic.with_severity(severity);

                self.diagnostics
                    .push(biome_diagnostics::serde::Diagnostic::new(error));
            }
        }

        ControlFlow::<Never>::Continue(())
    }

    pub(crate) fn into_result(
        self,
        parse_diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
        analyzer_diagnostics: Vec<biome_diagnostics::Error>,
    ) -> LintResults {
        let mut diagnostics = parse_diagnostics;
        let errors = diagnostics
            .iter()
            .filter(|diag| diag.severity() <= Severity::Error)
            .count();

        diagnostics.extend(self.diagnostics);

        diagnostics.extend(
            analyzer_diagnostics
                .into_iter()
                .map(biome_diagnostics::serde::Diagnostic::new)
                .collect::<Vec<_>>(),
        );
        let skipped_diagnostics = self
            .diagnostic_count
            .saturating_sub(diagnostics.len() as u32);

        LintResults {
            errors,
            skipped_diagnostics,
            diagnostics,
        }
    }
}

pub(crate) struct CodeActionsParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) range: Option<TextRange>,
    pub(crate) workspace: &'a WorkspaceSettingsHandle,
    pub(crate) path: &'a BiomePath,
    pub(crate) dependency_graph: Arc<DependencyGraph>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) language: DocumentFileSource,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: Vec<RuleSelector>,
    pub(crate) plugins: AnalyzerPluginVec,
}

type Lint = fn(LintParams) -> LintResults;
type CodeActions = fn(CodeActionsParams) -> PullActionsResult;
type FixAll = fn(FixAllParams) -> Result<FixFileResult, WorkspaceError>;
type Rename = fn(&BiomePath, AnyParse, TextSize, String) -> Result<RenameResult, WorkspaceError>;

#[derive(Default)]
pub struct AnalyzerCapabilities {
    /// It lints a file
    pub(crate) lint: Option<Lint>,
    /// It extracts code actions for a file
    pub(crate) code_actions: Option<CodeActions>,
    /// Applies fixes to a file
    pub(crate) fix_all: Option<FixAll>,
    /// It renames a binding inside a file
    pub(crate) rename: Option<Rename>,
}

type Format = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError>;
type FormatRange = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    WorkspaceSettingsHandle,
    TextRange,
) -> Result<Printed, WorkspaceError>;
type FormatOnType = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    WorkspaceSettingsHandle,
    TextSize,
) -> Result<Printed, WorkspaceError>;

#[derive(Default)]
pub(crate) struct FormatterCapabilities {
    /// It formats a file
    pub(crate) format: Option<Format>,
    /// It formats a portion of text of a file
    pub(crate) format_range: Option<FormatRange>,
    /// It formats a file while typing
    pub(crate) format_on_type: Option<FormatOnType>,
}

type Enabled = fn(&Utf8Path, &WorkspaceSettingsHandle) -> bool;

type Search = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &GritQuery,
    WorkspaceSettingsHandle,
) -> Result<Vec<TextRange>, WorkspaceError>;

#[derive(Default)]
pub(crate) struct SearchCapabilities {
    /// It searches through a file
    pub(crate) search: Option<Search>,
}

#[derive(Default)]
pub(crate) struct EnabledForPath {
    pub(crate) formatter: Option<Enabled>,
    pub(crate) linter: Option<Enabled>,
    pub(crate) assist: Option<Enabled>,
    pub(crate) search: Option<Enabled>,
}

/// Main trait to use to add a new language to Biome
pub(crate) trait ExtensionHandler {
    /// Capabilities that can applied to a file
    fn capabilities(&self) -> Capabilities {
        Capabilities::default()
    }
}

/// Features available for each language
pub(crate) struct Features {
    js: JsFileHandler,
    json: JsonFileHandler,
    css: CssFileHandler,
    astro: AstroFileHandler,
    vue: VueFileHandler,
    svelte: SvelteFileHandler,
    unknown: UnknownFileHandler,
    graphql: GraphqlFileHandler,
    html: HtmlFileHandler,
    grit: GritFileHandler,
}

impl Features {
    pub(crate) fn new() -> Self {
        Features {
            js: JsFileHandler {},
            json: JsonFileHandler {},
            css: CssFileHandler {},
            astro: AstroFileHandler {},
            vue: VueFileHandler {},
            svelte: SvelteFileHandler {},
            graphql: GraphqlFileHandler {},
            html: HtmlFileHandler {},
            grit: GritFileHandler {},
            unknown: UnknownFileHandler::default(),
        }
    }

    /// Returns the [Capabilities] associated with a [BiomePath]
    pub(crate) fn get_capabilities(
        &self,
        path: &Utf8Path,
        language_hint: DocumentFileSource,
    ) -> Capabilities {
        match DocumentFileSource::from_path(path).or(language_hint) {
            DocumentFileSource::Js(source) => match source.as_embedding_kind() {
                EmbeddingKind::Astro => self.astro.capabilities(),
                EmbeddingKind::Vue => self.vue.capabilities(),
                EmbeddingKind::Svelte => self.svelte.capabilities(),
                EmbeddingKind::None => self.js.capabilities(),
            },
            DocumentFileSource::Json(_) => self.json.capabilities(),
            DocumentFileSource::Css(_) => self.css.capabilities(),
            DocumentFileSource::Graphql(_) => self.graphql.capabilities(),
            DocumentFileSource::Html(_) => self.html.capabilities(),
            DocumentFileSource::Grit(_) => self.grit.capabilities(),
            DocumentFileSource::Unknown => self.unknown.capabilities(),
        }
    }
}

/// Checks whether a diagnostic coming from the analyzer is an [error](Severity::Error)
///
/// The function checks the diagnostic against the current configured rules.
pub(crate) fn is_diagnostic_error(
    diagnostic: &'_ AnalyzerDiagnostic,
    rules: Option<&'_ Rules>,
) -> bool {
    let severity = diagnostic
        .category()
        .filter(|category| {
            category.name().starts_with("lint/") || category.name().starts_with("assist/")
        })
        .map_or_else(
            || diagnostic.severity(),
            |category| {
                rules
                    .and_then(|rules| {
                        rules.get_severity_from_category(category, diagnostic.severity())
                    })
                    .unwrap_or(Severity::Warning)
            },
        );

    severity >= Severity::Error
}

/// Parse the "lang" attribute from the opening tag of the "\<script\>" block in Svelte or Vue files.
/// This function will return the language based on the existence or the value of the "lang" attribute.
/// We use the JSX parser at the moment to parse the opening tag. So the opening tag should be first
/// matched by regular expressions.
///
// TODO: We should change the parser when HTMLish languages are supported.
pub(crate) fn parse_lang_from_script_opening_tag(
    script_opening_tag: &str,
) -> (Language, LanguageVariant) {
    parse(
        script_opening_tag,
        JsFileSource::jsx(),
        JsParserOptions::default(),
    )
    .try_tree()
    .and_then(|tree| {
        tree.as_js_module()?.items().into_iter().find_map(|item| {
            let expression = item
                .as_any_js_statement()?
                .as_js_expression_statement()?
                .expression()
                .ok()?;
            let tag = expression.as_jsx_tag_expression()?.tag().ok()?;
            let opening_element = tag.as_jsx_element()?.opening_element().ok()?;
            let lang_attribute = opening_element.attributes().find_by_name("lang")?;
            let attribute_value = lang_attribute.initializer()?.value().ok()?;
            let attribute_inner_string =
                attribute_value.as_jsx_string()?.inner_string_text().ok()?;
            match attribute_inner_string.text() {
                "ts" => Some((
                    Language::TypeScript {
                        definition_file: false,
                    },
                    LanguageVariant::Standard,
                )),
                "tsx" => Some((
                    Language::TypeScript {
                        definition_file: false,
                    },
                    LanguageVariant::Jsx,
                )),
                "jsx" => Some((Language::JavaScript, LanguageVariant::Jsx)),
                "js" => Some((Language::JavaScript, LanguageVariant::Standard)),
                _ => None,
            }
        })
    })
    .map_or((Language::JavaScript, LanguageVariant::Standard), |lang| {
        lang
    })
}

pub(crate) fn search(
    path: &BiomePath,
    _file_source: &DocumentFileSource,
    parse: AnyParse,
    query: &GritQuery,
    _settings: WorkspaceSettingsHandle,
) -> Result<Vec<TextRange>, WorkspaceError> {
    let result = query
        .execute(GritTargetFile {
            path: path.to_path_buf(),
            parse,
        })
        .map_err(|err| {
            WorkspaceError::SearchError(SearchError::QueryError(QueryDiagnostic(err.to_string())))
        })?;

    let matches = result
        .effects
        .into_iter()
        .flat_map(|result| match result {
            GritQueryEffect::Match(m) => m.ranges,
            _ => Vec::new(),
        })
        .map(|range| TextRange::new(range.start_byte.into(), range.end_byte.into()))
        .collect();

    Ok(matches)
}

#[test]
fn test_svelte_script_lang() {
    const SVELTE_JS_SCRIPT_OPENING_TAG: &str = r#"<script>"#;
    const SVELTE_TS_SCRIPT_OPENING_TAG: &str = r#"<script lang="ts">"#;
    const SVELTE_CONTEXT_MODULE_JS_SCRIPT_OPENING_TAG: &str = r#"<script context="module">"#;
    const SVELTE_CONTEXT_MODULE_TS_SCRIPT_OPENING_TAG: &str =
        r#"<script context="module" lang="ts">"#;

    assert!(
        parse_lang_from_script_opening_tag(SVELTE_JS_SCRIPT_OPENING_TAG)
            .0
            .is_javascript()
    );
    assert!(
        parse_lang_from_script_opening_tag(SVELTE_TS_SCRIPT_OPENING_TAG)
            .0
            .is_typescript()
    );
    assert!(
        parse_lang_from_script_opening_tag(SVELTE_CONTEXT_MODULE_JS_SCRIPT_OPENING_TAG)
            .0
            .is_javascript()
    );
    assert!(
        parse_lang_from_script_opening_tag(SVELTE_CONTEXT_MODULE_TS_SCRIPT_OPENING_TAG)
            .0
            .is_typescript()
    );
}

/// Type meant to register all the syntax rules for each language supported by Biome
///
/// When a new language is introduced, it must be implemented it. Syntax rules aren't negotiable via configuration, so it's safe
/// to pull all of them.
#[derive(Default, Debug)]
struct SyntaxVisitor<'a> {
    pub(crate) enabled_rules: Vec<RuleFilter<'a>>,
}

impl RegistryVisitor<JsLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

impl RegistryVisitor<JsonLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

impl RegistryVisitor<CssLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

impl RegistryVisitor<GraphqlLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

/// Type meant to register all the lint rules for each language supported by Biome
///
#[derive(Debug)]
struct LintVisitor<'a, 'b> {
    pub(crate) enabled_rules: FxHashSet<RuleFilter<'a>>,
    pub(crate) disabled_rules: FxHashSet<RuleFilter<'a>>,
    // lint_params: &'b LintParams<'a>,
    only: Option<&'b [RuleSelector]>,
    skip: Option<&'b [RuleSelector]>,
    settings: Option<&'b Settings>,
    path: Option<&'b Utf8Path>,
    project_layout: Arc<ProjectLayout>,
    analyzer_options: &'b mut AnalyzerOptions,
}

impl<'a, 'b> LintVisitor<'a, 'b> {
    pub(crate) fn new(
        only: Option<&'b [RuleSelector]>,
        skip: Option<&'b [RuleSelector]>,
        settings: Option<&'b Settings>,
        path: Option<&'b Utf8Path>,
        project_layout: Arc<ProjectLayout>,
        analyzer_options: &'b mut AnalyzerOptions,
    ) -> Self {
        Self {
            enabled_rules: Default::default(),
            disabled_rules: Default::default(),
            only,
            skip,
            settings,
            path,
            project_layout,
            analyzer_options,
        }
    }

    /// It loops over the domains of the current rule, and check each domain specify
    /// a dependency.
    ///
    /// Returns `true` if the rule was enabled, `false` otherwise
    fn record_rule_from_manifest<R, L>(&mut self, rule_filter: RuleFilter<'static>)
    where
        L: biome_rowan::Language,
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let path = self.path.expect("File path");
        let no_only = self.only.is_some_and(|only| only.is_empty());
        let no_domains = self
            .settings
            .and_then(|settings| settings.as_linter_domains(path))
            .is_none_or(|d| d.is_empty());
        if !(no_only && no_domains) {
            return;
        }

        if let Some((_, manifest)) = self.project_layout.get_node_manifest_for_path(path) {
            for domain in R::METADATA.domains {
                self.analyzer_options
                    .push_globals(domain.globals().iter().map(|s| Box::from(*s)).collect());

                for (dependency, range) in domain.manifest_dependencies() {
                    if manifest.matches_dependency(dependency, range) {
                        self.enabled_rules.insert(rule_filter);
                    }
                }
            }
        }
    }

    /// It inspects the [RuleDomain] of the configuration, and if the current rule belongs to at least a configured domain, it's enabled.
    ///
    /// As per business logic, rules that have domains can be recommended, however they shouldn't be enabled when `linter.rules.recommended` is `true`.
    ///
    /// This means that
    fn record_rule_from_domains<R, L>(&mut self, rule_filter: RuleFilter<'static>)
    where
        L: biome_rowan::Language,
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let no_only = self.only.is_some_and(|only| only.is_empty());

        if !no_only {
            return;
        }

        let domains = self
            .settings
            .and_then(|settings| settings.as_linter_domains(self.path.expect("File path")));

        // no domains, no need to record the rule
        if domains.as_ref().is_none_or(|d| d.is_empty()) {
            return;
        }

        // If the rule is recommended, and it has some domains, it should be disabled, but only if the configuration doesn't enable some domains.
        if R::METADATA.recommended
            && !R::METADATA.domains.is_empty()
            && domains.as_ref().is_none_or(|d| d.is_empty())
        {
            self.disabled_rules.insert(rule_filter);
            return;
        }

        for rule_domain in R::METADATA.domains {
            if let Some((configured_domain, configured_domain_value)) = domains
                .as_ref()
                .and_then(|domains| domains.get_key_value(rule_domain))
            {
                match configured_domain_value {
                    RuleDomainValue::All => {
                        self.enabled_rules.insert(rule_filter);

                        self.analyzer_options.push_globals(
                            configured_domain
                                .globals()
                                .iter()
                                .map(|s| Box::from(*s))
                                .collect::<Vec<_>>(),
                        );
                    }
                    RuleDomainValue::None => {
                        self.disabled_rules.insert(rule_filter);
                    }
                    RuleDomainValue::Recommended => {
                        if R::METADATA.recommended {
                            self.enabled_rules.insert(rule_filter);

                            self.analyzer_options.push_globals(
                                configured_domain
                                    .globals()
                                    .iter()
                                    .map(|s| Box::from(*s))
                                    .collect::<Vec<_>>(),
                            );
                        }
                    }
                }
            }
        }
    }

    fn finish(mut self) -> (FxHashSet<RuleFilter<'a>>, FxHashSet<RuleFilter<'a>>) {
        let has_only_filter = self.only.map_or(true, |only| !only.is_empty());
        let rules = self
            .settings
            .and_then(|settings| settings.as_linter_rules(self.path.expect("Path to be set")))
            .unwrap_or_default();
        if !has_only_filter {
            self.enabled_rules.extend(rules.as_enabled_rules());
            self.disabled_rules.extend(rules.as_disabled_rules());
        }
        (self.enabled_rules, self.disabled_rules)
    }

    fn push_rule<R, L>(&mut self, rule_filter: Option<RuleFilter<'static>>)
    where
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
        L: biome_rowan::Language,
    {
        if let Some(rule_filter) = rule_filter {
            if rule_filter.match_rule::<R>() {
                // first we want to register rules via "magic default"
                self.record_rule_from_manifest::<R, L>(rule_filter);
                // then we want to register rules
                self.record_rule_from_domains::<R, L>(rule_filter);
            }
        };

        // Do not report unused suppression comment diagnostics if:
        // - it is a syntax-only analyzer pass, or
        // - if a single rule is run.
        if let Some(only) = self.only {
            for selector in only {
                let filter = RuleFilter::from(selector);
                if filter.match_rule::<R>() && filter.match_group::<R::Group>() {
                    self.enabled_rules.insert(filter);
                }
            }
        }
        if let Some(skip) = self.skip {
            for selector in skip {
                let filter = RuleFilter::from(selector);
                if filter.match_rule::<R>() && filter.match_group::<R::Group>() {
                    self.disabled_rules.insert(filter);
                }
            }
        }
    }
}

impl RegistryVisitor<JsLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = JsLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            js_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}
impl RegistryVisitor<JsonLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = JsonLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            json_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

impl RegistryVisitor<CssLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = CssLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            css_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

impl RegistryVisitor<GraphqlLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = GraphqlLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            graphql_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

struct AssistsVisitor<'a, 'b> {
    settings: Option<&'b Settings>,
    enabled_rules: Vec<RuleFilter<'a>>,
    disabled_rules: Vec<RuleFilter<'a>>,
    only: Option<&'b [RuleSelector]>,
    skip: Option<&'b [RuleSelector]>,
    path: Option<&'b Utf8Path>,
}

impl<'a, 'b> AssistsVisitor<'a, 'b> {
    pub(crate) fn new(
        only: Option<&'b [RuleSelector]>,
        skip: Option<&'b [RuleSelector]>,
        settings: Option<&'b Settings>,
        path: Option<&'b Utf8Path>,
    ) -> Self {
        Self {
            enabled_rules: vec![],
            disabled_rules: vec![],
            settings,
            path,
            only,
            skip,
        }
    }

    pub(crate) fn push_rule<R, L>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        // We deem refactors **safe**, other assists aren't safe
        if R::Group::NAME != "source" {
            return;
        }

        // Do not report unused suppression comment diagnostics if:
        // - it is a syntax-only analyzer pass, or
        // - if a single rule is run.
        if let Some(only) = self.only {
            for selector in only {
                let filter = RuleFilter::from(selector);
                if filter.match_rule::<R>() {
                    self.enabled_rules.push(filter)
                }
            }
        }

        if let Some(skip) = self.skip {
            for selector in skip {
                let filter = RuleFilter::from(selector);
                if filter.match_rule::<R>() {
                    self.disabled_rules.push(filter)
                }
            }
        }
    }

    fn finish(mut self) -> (Vec<RuleFilter<'a>>, Vec<RuleFilter<'a>>) {
        let has_only_filter = self.only.map_or(true, |only| !only.is_empty());
        let rules = self
            .settings
            .and_then(|settings| settings.as_assist_actions(self.path.expect("Path to be set")))
            .unwrap_or_default();
        if !has_only_filter {
            self.enabled_rules.extend(rules.as_enabled_rules());
            self.disabled_rules.extend(rules.as_disabled_rules());
        }
        (self.enabled_rules, self.disabled_rules)
    }
}

impl RegistryVisitor<JsLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

impl RegistryVisitor<JsonLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

impl RegistryVisitor<CssLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

impl RegistryVisitor<GraphqlLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

pub(crate) struct AnalyzerVisitorBuilder<'a> {
    settings: Option<&'a Settings>,
    only: Option<&'a [RuleSelector]>,
    skip: Option<&'a [RuleSelector]>,
    path: Option<&'a Utf8Path>,
    enabled_rules: Option<&'a [RuleSelector]>,
    project_layout: Arc<ProjectLayout>,
    analyzer_options: AnalyzerOptions,
}

impl<'b> AnalyzerVisitorBuilder<'b> {
    pub(crate) fn new(settings: Option<&'b Settings>, analyzer_options: AnalyzerOptions) -> Self {
        Self {
            settings,
            only: None,
            skip: None,
            path: None,
            enabled_rules: None,
            project_layout: Default::default(),
            analyzer_options,
        }
    }

    #[must_use]
    pub(crate) fn with_only(mut self, only: &'b [RuleSelector]) -> Self {
        self.only = Some(only);
        self
    }

    #[must_use]
    pub(crate) fn with_skip(mut self, skip: &'b [RuleSelector]) -> Self {
        self.skip = Some(skip);
        self
    }

    #[must_use]
    pub(crate) fn with_path(mut self, path: &'b Utf8Path) -> Self {
        self.path = Some(path);
        self
    }

    #[must_use]
    pub(crate) fn with_enabled_rules(mut self, enabled_rules: &'b [RuleSelector]) -> Self {
        self.enabled_rules = Some(enabled_rules);
        self
    }

    #[must_use]
    pub(crate) fn with_project_layout(mut self, project_layout: Arc<ProjectLayout>) -> Self {
        self.project_layout = project_layout;
        self
    }

    #[must_use]
    pub(crate) fn finish(self) -> (Vec<RuleFilter<'b>>, Vec<RuleFilter<'b>>, AnalyzerOptions) {
        let mut analyzer_options = self.analyzer_options;
        let mut disabled_rules = vec![];
        let mut enabled_rules: Vec<_> = self
            .enabled_rules
            .map(|enabled_rules| {
                enabled_rules
                    .iter()
                    .map(RuleFilter::from)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let mut syntax = SyntaxVisitor::default();

        biome_js_analyze::visit_registry(&mut syntax);
        biome_css_analyze::visit_registry(&mut syntax);
        biome_json_analyze::visit_registry(&mut syntax);
        biome_graphql_analyze::visit_registry(&mut syntax);
        enabled_rules.extend(syntax.enabled_rules);

        let mut lint = LintVisitor::new(
            self.only,
            self.skip,
            self.settings,
            self.path,
            self.project_layout,
            &mut analyzer_options,
        );

        biome_js_analyze::visit_registry(&mut lint);
        biome_css_analyze::visit_registry(&mut lint);
        biome_json_analyze::visit_registry(&mut lint);
        biome_graphql_analyze::visit_registry(&mut lint);
        let (linter_enabled_rules, linter_disabled_rules) = lint.finish();
        enabled_rules.extend(linter_enabled_rules);
        disabled_rules.extend(linter_disabled_rules);

        let mut assist = AssistsVisitor::new(self.only, self.skip, self.settings, self.path);

        biome_js_analyze::visit_registry(&mut assist);
        biome_css_analyze::visit_registry(&mut assist);
        biome_json_analyze::visit_registry(&mut assist);
        biome_graphql_analyze::visit_registry(&mut assist);
        let (assists_enabled_rules, assists_disabled_rules) = assist.finish();
        enabled_rules.extend(assists_enabled_rules);
        disabled_rules.extend(assists_disabled_rules);

        (enabled_rules, disabled_rules, analyzer_options)
    }
}

#[test]
fn test_vue_script_lang() {
    const VUE_JS_SCRIPT_OPENING_TAG: &str = r#"<script>"#;
    const VUE_TS_SCRIPT_OPENING_TAG: &str = r#"<script lang="ts">"#;
    const VUE_TSX_SCRIPT_OPENING_TAG: &str = r#"<script lang="tsx">"#;
    const VUE_JSX_SCRIPT_OPENING_TAG: &str = r#"<script lang="jsx">"#;
    const VUE_SETUP_JS_SCRIPT_OPENING_TAG: &str = r#"<script setup>"#;
    const VUE_SETUP_TS_SCRIPT_OPENING_TAG: &str = r#"<script setup lang="ts">"#;

    assert!(
        parse_lang_from_script_opening_tag(VUE_JS_SCRIPT_OPENING_TAG)
            .0
            .is_javascript()
    );
    assert!(
        parse_lang_from_script_opening_tag(VUE_JS_SCRIPT_OPENING_TAG)
            .1
            .is_standard()
    );
    assert!(
        parse_lang_from_script_opening_tag(VUE_TS_SCRIPT_OPENING_TAG)
            .0
            .is_typescript()
    );
    assert!(
        parse_lang_from_script_opening_tag(VUE_TS_SCRIPT_OPENING_TAG)
            .1
            .is_standard()
    );
    assert!(
        parse_lang_from_script_opening_tag(VUE_JSX_SCRIPT_OPENING_TAG)
            .0
            .is_javascript()
    );
    assert!(
        parse_lang_from_script_opening_tag(VUE_JSX_SCRIPT_OPENING_TAG)
            .1
            .is_jsx()
    );
    assert!(
        parse_lang_from_script_opening_tag(VUE_TSX_SCRIPT_OPENING_TAG)
            .0
            .is_typescript()
    );
    assert!(
        parse_lang_from_script_opening_tag(VUE_SETUP_JS_SCRIPT_OPENING_TAG)
            .0
            .is_javascript()
    );
    assert!(
        parse_lang_from_script_opening_tag(VUE_SETUP_TS_SCRIPT_OPENING_TAG)
            .0
            .is_typescript()
    );
}
