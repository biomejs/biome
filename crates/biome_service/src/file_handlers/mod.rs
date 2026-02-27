use self::{
    css::CssFileHandler, javascript::JsFileHandler, json::JsonFileHandler,
    unknown::UnknownFileHandler,
};
use crate::WorkspaceError;
use crate::diagnostics::{QueryDiagnostic, SearchError};
pub use crate::file_handlers::astro::AstroFileHandler;
use crate::file_handlers::graphql::GraphqlFileHandler;
use crate::file_handlers::ignore::IgnoreFileHandler;
pub use crate::file_handlers::svelte::SvelteFileHandler;
pub use crate::file_handlers::vue::VueFileHandler;
use crate::settings::{Settings, SettingsWithEditor};
use crate::utils::growth_guard::GrowthGuard;
use crate::workspace::document::services::embedded_bindings::EmbeddedBuilder;
use crate::workspace::{
    AnyEmbeddedSnippet, CodeAction, DocumentServices, FixAction, FixFileMode, FixFileResult,
    GetSyntaxTreeResult, PullActionsResult, PullDiagnosticsAndActionsResult, RenameResult,
};
use biome_analyze::{
    AnalyzerAction, AnalyzerDiagnostic, AnalyzerOptions, AnalyzerPluginVec, AnalyzerSignal,
    ControlFlow, GroupCategory, Never, Queryable, RegistryVisitor, Rule, RuleCategories,
    RuleCategory, RuleError, RuleFilter, RuleGroup,
};
use biome_configuration::Rules;
use biome_configuration::analyzer::{AnalyzerSelector, RuleDomainValue};
use biome_configuration::vcs::{GIT_IGNORE_FILE_NAME, IGNORE_FILE_NAME};
use biome_console::fmt::Formatter;
use biome_css_analyze::METADATA as css_metadata;
use biome_css_syntax::{CssFileSource, CssLanguage};
use biome_diagnostics::{Applicability, Diagnostic, DiagnosticExt, Error, Severity, category};
use biome_formatter::{FormatContext, FormatResult, Formatted, Printed};
use biome_fs::BiomePath;
use biome_graphql_analyze::METADATA as graphql_metadata;
use biome_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use biome_grit_patterns::{GritQuery, GritQueryEffect, GritTargetFile};
use biome_grit_syntax::file_source::GritFileSource;
use biome_html_syntax::{HtmlFileSource, HtmlLanguage};
use biome_js_analyze::METADATA as js_metadata;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsModuleItem, EmbeddingKind, JsFileSource, JsLanguage, JsxAttribute, JsxAttributeList,
    Language, LanguageVariant, TextRange, TextSize,
};
use biome_json_analyze::METADATA as json_metadata;
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_markdown_syntax::MdFileSource;
use biome_module_graph::ModuleGraph;
use biome_package::PackageJson;
use biome_parser::AnyParse;
use biome_project_layout::ProjectLayout;
use biome_rowan::{FileSourceError, NodeCache, SendNode, SyntaxNode, TokenText};
use biome_string_case::StrLikeExtension;
use camino::Utf8Path;
use either::Either;
use grit::GritFileHandler;
use html::HtmlFileHandler;
pub use javascript::JsFormatterSettings;
use markdown::MarkdownFileHandler;
use rustc_hash::FxHashSet;
use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::Arc;
use tracing::instrument;

pub mod astro;
pub(crate) mod css;
pub(crate) mod graphql;
pub(crate) mod grit;
pub(crate) mod html;
mod ignore;
pub(crate) mod javascript;
pub(crate) mod json;
pub(crate) mod markdown;
pub mod svelte;
mod unknown;
pub mod vue;

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
    Markdown(MdFileSource),
    // Ignore files
    Ignore,
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

impl From<MdFileSource> for DocumentFileSource {
    fn from(value: MdFileSource) -> Self {
        Self::Markdown(value)
    }
}

impl From<&Utf8Path> for DocumentFileSource {
    fn from(path: &Utf8Path) -> Self {
        Self::from_path(path, false)
    }
}

impl DocumentFileSource {
    fn try_from_well_known(
        path: &Utf8Path,
        experimental_full_html_support: bool,
    ) -> Result<Self, FileSourceError> {
        if let Ok(file_source) = JsonFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }
        if experimental_full_html_support {
            if let Ok(file_source) = HtmlFileSource::try_from_well_known(path) {
                return Ok(file_source.into());
            }
            if let Ok(file_source) = JsFileSource::try_from_well_known(path) {
                return Ok(file_source.into());
            }
        } else {
            if let Ok(file_source) = JsFileSource::try_from_well_known(path) {
                return Ok(file_source.into());
            }
            if let Ok(file_source) = HtmlFileSource::try_from_well_known(path) {
                return Ok(file_source.into());
            }
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
    pub fn from_well_known(path: &Utf8Path, experimental_full_html_support: bool) -> Self {
        Self::try_from_well_known(path, experimental_full_html_support).unwrap_or(Self::Unknown)
    }

    fn try_from_extension(
        extension: &str,
        experimental_full_html_support: bool,
    ) -> Result<Self, FileSourceError> {
        // Order here is important
        if experimental_full_html_support {
            if let Ok(file_source) = HtmlFileSource::try_from_extension(extension) {
                return Ok(file_source.into());
            }
            if let Ok(file_source) = JsFileSource::try_from_extension(extension) {
                return Ok(file_source.into());
            }
        } else {
            if let Ok(file_source) = JsFileSource::try_from_extension(extension) {
                return Ok(file_source.into());
            }

            if let Ok(file_source) = HtmlFileSource::try_from_extension(extension) {
                return Ok(file_source.into());
            }
        }

        if let Ok(file_source) = JsonFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = CssFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = GraphqlFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }

        if let Ok(file_source) = GritFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = MdFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        Err(FileSourceError::UnknownExtension)
    }

    /// Returns the document file source corresponding to this file extension
    pub fn from_extension(extension: &str, experimental_full_html_support: bool) -> Self {
        Self::try_from_extension(extension, experimental_full_html_support).unwrap_or(Self::Unknown)
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
        if let Ok(file_source) = HtmlFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = GritFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = MdFileSource::try_from_language_id(language_id) {
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
        Self::try_from_language_id(language_id).unwrap_or(Self::Unknown)
    }

    pub(crate) fn try_from_path(
        path: &Utf8Path,
        experimental_full_html_support: bool,
    ) -> Result<Self, FileSourceError> {
        if let Ok(file_source) = Self::try_from_well_known(path, experimental_full_html_support) {
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
            // Ignore files are extensionless files, so they need to be handled in particular way
            Some(filename) if filename == GIT_IGNORE_FILE_NAME || filename == IGNORE_FILE_NAME => {
                return Ok(Self::Ignore);
            }
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

        Self::try_from_extension(extension.as_ref(), experimental_full_html_support)
    }

    /// Returns the document file source corresponding to the file path
    pub fn from_path(path: &Utf8Path, experimental_full_html_support: bool) -> Self {
        Self::try_from_path(path, experimental_full_html_support).unwrap_or(Self::Unknown)
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
    pub fn or(self, other: Self) -> Self {
        if self != Self::Unknown { self } else { other }
    }

    pub const fn is_javascript_like(&self) -> bool {
        matches!(self, Self::Js(_))
    }

    pub const fn is_json_like(&self) -> bool {
        matches!(self, Self::Json(_))
    }

    pub const fn is_css_like(&self) -> bool {
        matches!(self, Self::Css(_))
    }

    pub fn to_js_file_source(&self) -> Option<JsFileSource> {
        match self {
            Self::Js(file_source) => Some(*file_source),
            _ => None,
        }
    }

    pub fn to_json_file_source(&self) -> Option<JsonFileSource> {
        match self {
            Self::Json(json) => Some(*json),
            _ => None,
        }
    }

    pub fn to_graphql_file_source(&self) -> Option<GraphqlFileSource> {
        match self {
            Self::Graphql(graphql) => Some(*graphql),
            _ => None,
        }
    }

    pub fn to_grit_file_source(&self) -> Option<GritFileSource> {
        match self {
            Self::Grit(grit) => Some(*grit),
            _ => None,
        }
    }

    pub fn to_css_file_source(&self) -> Option<CssFileSource> {
        match self {
            Self::Css(css) => Some(*css),
            _ => None,
        }
    }

    pub fn to_html_file_source(&self) -> Option<HtmlFileSource> {
        match self {
            Self::Html(html) => Some(*html),
            _ => None,
        }
    }

    pub fn to_markdown_file_source(&self) -> Option<MdFileSource> {
        match self {
            Self::Markdown(markdown) => Some(*markdown),
            _ => None,
        }
    }

    /// The file can be parsed
    pub fn can_parse(path: &Utf8Path) -> bool {
        let file_source = Self::from(path);
        match file_source {
            Self::Js(_)
            | Self::Css(_)
            | Self::Graphql(_)
            | Self::Json(_)
            | Self::Html(_)
            | Self::Grit(_)
            | Self::Markdown(_) => true,
            Self::Ignore => false,
            Self::Unknown => false,
        }
    }

    /// The file can be read from the file system
    pub fn can_read(path: &Utf8Path) -> bool {
        let file_source = Self::from(path);
        match file_source {
            Self::Js(_)
            | Self::Css(_)
            | Self::Graphql(_)
            | Self::Json(_)
            | Self::Html(_)
            | Self::Grit(_)
            | Self::Markdown(_) => true,
            Self::Ignore => true,
            Self::Unknown => false,
        }
    }

    /// Whether this file can contain embedded nodes
    pub fn can_contain_embeds(path: &Utf8Path, experimental_full_html_support: bool) -> bool {
        let file_source = Self::from_path(path, experimental_full_html_support);
        match file_source {
            Self::Html(_) | Self::Js(_) => true,
            Self::Css(_)
            | Self::Graphql(_)
            | Self::Json(_)
            | Self::Grit(_)
            | Self::Markdown(_)
            | Self::Ignore
            | Self::Unknown => false,
        }
    }
}

impl std::fmt::Display for DocumentFileSource {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Js(js) => {
                let is_jsx = js.is_jsx();
                if js.is_typescript() {
                    if is_jsx {
                        write!(fmt, "TSX")
                    } else {
                        write!(fmt, "TypeScript")
                    }
                } else if is_jsx {
                    write!(fmt, "JSX")
                } else {
                    write!(fmt, "JavaScript")
                }
            }
            Self::Json(json) => {
                if json.allow_comments() {
                    write!(fmt, "JSONC")
                } else {
                    write!(fmt, "JSON")
                }
            }
            Self::Css(_) => write!(fmt, "CSS"),
            Self::Graphql(_) => write!(fmt, "GraphQL"),
            Self::Html(_) => write!(fmt, "HTML"),
            Self::Grit(_) => write!(fmt, "Grit"),
            Self::Markdown(_) => write!(fmt, "Markdown"),
            Self::Ignore => write!(fmt, "Ignore"),
            Self::Unknown => write!(fmt, "Unknown"),
        }
    }
}

impl biome_console::fmt::Display for DocumentFileSource {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        fmt.write_fmt(format_args!("{self}"))
    }
}

pub struct FixAllParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    /// Whether it should format the code action
    pub(crate) should_format: bool,
    pub(crate) biome_path: &'a BiomePath,
    pub(crate) module_graph: Arc<ModuleGraph>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) document_file_source: DocumentFileSource,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) rule_categories: RuleCategories,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) document_services: &'a DocumentServices,
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

#[derive(Default)]
pub struct ParseEmbedResult {
    pub(crate) nodes: Vec<(AnyEmbeddedSnippet, DocumentFileSource)>,
}

type Parse =
    fn(&BiomePath, DocumentFileSource, &str, &SettingsWithEditor, &mut NodeCache) -> ParseResult;
type ParseEmbeddedNodes = fn(
    &AnyParse,
    &BiomePath,
    &DocumentFileSource,
    &SettingsWithEditor,
    &mut NodeCache,
    &mut EmbeddedBuilder,
) -> ParseEmbedResult;
#[derive(Default)]
pub struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,

    pub(crate) parse_embedded_nodes: Option<ParseEmbeddedNodes>,
}

type DebugSyntaxTree = fn(&BiomePath, AnyParse) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(AnyParse, TextSize) -> String;
type DebugFormatterIR = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
) -> Result<String, WorkspaceError>;
type DebugTypeInfo =
    fn(&BiomePath, Option<AnyParse>, Arc<ModuleGraph>) -> Result<String, WorkspaceError>;
type DebugRegisteredTypes = fn(&BiomePath, AnyParse) -> Result<String, WorkspaceError>;
type DebugSemanticModel = fn(&BiomePath, AnyParse) -> Result<String, WorkspaceError>;

#[derive(Default)]
pub struct DebugCapabilities {
    /// Prints the syntax tree
    pub(crate) debug_syntax_tree: Option<DebugSyntaxTree>,
    /// Prints the control flow graph
    pub(crate) debug_control_flow: Option<DebugControlFlow>,
    /// Prints the formatter IR
    pub(crate) debug_formatter_ir: Option<DebugFormatterIR>,
    /// Prints the type info
    pub(crate) debug_type_info: Option<DebugTypeInfo>,
    /// Prints the registered types
    pub(crate) debug_registered_types: Option<DebugRegisteredTypes>,
    /// Prints the binding/scope tree of the semantic model
    pub(crate) debug_semantic_model: Option<DebugSemanticModel>,
}

#[derive(Debug)]
pub(crate) struct LintParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) language: DocumentFileSource,
    pub(crate) path: &'a BiomePath,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) categories: RuleCategories,
    pub(crate) module_graph: Arc<ModuleGraph>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_selectors: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) pull_code_actions: bool,
    pub(crate) diagnostic_offset: Option<TextSize>,
    pub(crate) document_services: &'a DocumentServices,
    pub(crate) snippet_services: Option<&'a DocumentServices>,
}

pub(crate) struct DiagnosticsAndActionsParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) language: DocumentFileSource,
    pub(crate) path: &'a BiomePath,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) categories: RuleCategories,
    pub(crate) module_graph: Arc<ModuleGraph>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_selectors: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) diagnostic_offset: Option<TextSize>,
    pub(crate) document_services: &'a DocumentServices,
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
    pull_code_actions: bool,
    diagnostic_offset: Option<TextSize>,
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
                .settings
                .as_ref()
                .as_linter_rules(params.path.as_path()),
            pull_code_actions: params.pull_code_actions,
            diagnostic_offset: params.diagnostic_offset,
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

            if self.pull_code_actions {
                for action in signal.actions() {
                    if !action.is_suppression() {
                        diagnostic = diagnostic.add_code_suggestion(action.into());
                    }
                }
            }
            if let Some(offset) = &self.diagnostic_offset {
                diagnostic.add_diagnostic_offset(*offset);
            }

            let error = diagnostic.with_severity(severity);

            self.diagnostics
                .push(biome_diagnostics::serde::Diagnostic::new(error));
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

/// Use this type to process fix all actions
pub(crate) struct ProcessFixAll<'a> {
    fix_file_mode: &'a FixFileMode,
    errors: usize,
    rules: Option<Cow<'a, Rules>>,
    skipped_suggested_fixes: u32,
    actions: Vec<FixAction>,
    growth_guard: GrowthGuard,
}

impl<'a> ProcessFixAll<'a> {
    pub(crate) fn new(
        params: &'a FixAllParams,
        rules: Option<Cow<'a, Rules>>,
        syntax_len: u32,
    ) -> Self {
        Self {
            fix_file_mode: &params.fix_file_mode,
            errors: 0,
            rules,
            skipped_suggested_fixes: 0,
            actions: Vec::new(),
            growth_guard: GrowthGuard::new(syntax_len),
        }
    }

    /// Process the incoming signal from the analyzer. Tracks errors and actions based on the type of
    /// fix have been provided.
    pub(crate) fn process_signal<L: biome_rowan::Language>(
        &mut self,
        signal: &dyn AnalyzerSignal<L>,
    ) -> ControlFlow<AnalyzerAction<L>> {
        let current_diagnostic = signal.diagnostic();

        if let Some(diagnostic) = current_diagnostic.as_ref()
            && is_diagnostic_error(diagnostic, self.rules.as_deref())
        {
            self.errors += 1;
        }

        for action in signal.actions() {
            match self.fix_file_mode {
                FixFileMode::ApplySuppressions => {
                    if action.is_suppression() {
                        return ControlFlow::Break(action);
                    }
                }
                FixFileMode::SafeFixes => {
                    // suppression actions should not be part of safe fixes
                    if action.is_suppression() {
                        continue;
                    }
                    if action.applicability == Applicability::MaybeIncorrect {
                        self.skipped_suggested_fixes += 1;
                    }
                    if action.applicability == Applicability::Always {
                        self.errors = self.errors.saturating_sub(1);
                        return ControlFlow::Break(action);
                    }
                }
                FixFileMode::SafeAndUnsafeFixes => {
                    if action.is_suppression() {
                        continue;
                    }
                    if matches!(
                        action.applicability,
                        Applicability::Always | Applicability::MaybeIncorrect
                    ) {
                        self.errors = self.errors.saturating_sub(1);
                        return ControlFlow::Break(action);
                    }
                }
            }
        }

        ControlFlow::Continue(())
    }

    /// Applies the mutation of the `action`. The closure returns the new root and must return
    /// the length of the text that was replaced by the mutation.
    ///
    /// If `None` is returned, it means that there aren't any more mutations to apply.
    pub(crate) fn process_action<T, L>(
        &mut self,
        action: Option<AnalyzerAction<L>>,
        mut update_tree_return_text_len: T,
    ) -> Result<Option<()>, WorkspaceError>
    where
        T: FnMut(SyntaxNode<L>) -> Option<u32>,
        L: biome_rowan::Language,
    {
        match action {
            Some(action) => {
                if let (root, Some((range, _))) =
                    action.mutation.commit_with_text_range_and_edit(true)
                {
                    let Some(curr_len) = update_tree_return_text_len(root) else {
                        return Err(WorkspaceError::RuleError(
                            RuleError::ReplacedRootWithNonRootError {
                                rule_name: action.rule_name.map(|(group, rule)| {
                                    (Cow::Borrowed(group), Cow::Borrowed(rule))
                                }),
                            },
                        ));
                    };

                    self.actions.push(FixAction {
                        rule_name: action
                            .rule_name
                            .map(|(group, rule)| (Cow::Borrowed(group), Cow::Borrowed(rule))),
                        range,
                    });

                    // Check for runaway edit growth
                    if !self.growth_guard.check(curr_len) {
                        // In order to provide a useful diagnostic, we want to flag the rules that caused the conflict.
                        // We can do this by inspecting the last few fixes that were applied.
                        // We limit it to the last 10 fixes. If there is a chain of conflicting fixes longer than that, something is **really** fucked up.

                        let mut seen_rules = HashSet::new();
                        for action in self.actions.iter().rev().take(10) {
                            if let Some((group, rule)) = action.rule_name.as_ref() {
                                seen_rules.insert((group.clone(), rule.clone()));
                            }
                        }

                        return Err(WorkspaceError::RuleError(
                            RuleError::ConflictingRuleFixesError {
                                rules: seen_rules.into_iter().collect(),
                            },
                        ));
                    };
                };

                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    /// Finish processing the fix all actions. Returns the result of the fix-all actions. The `format_tree`
    /// is a closure that must return the new code (formatted, if needed).
    pub(crate) fn finish<F, C>(self, format_tree: F) -> Result<FixFileResult, WorkspaceError>
    where
        F: FnOnce() -> Result<Either<FormatResult<Formatted<C>>, String>, WorkspaceError>,
        C: FormatContext,
    {
        let code = match format_tree()? {
            Either::Left(printed) => printed?.print()?.into_code(),
            Either::Right(string) => string,
        };
        Ok(FixFileResult {
            code,
            skipped_suggested_fixes: self.skipped_suggested_fixes,
            actions: self.actions,
            errors: self.errors,
        })
    }
}

pub(crate) struct ProcessDiagnosticsAndActions {
    diagnostics: Vec<(biome_diagnostics::serde::Diagnostic, Vec<CodeAction>)>,
    diagnostic_offset: Option<TextSize>,
}

impl ProcessDiagnosticsAndActions {
    pub(crate) fn new(diagnostic_offset: Option<TextSize>) -> Self {
        Self {
            diagnostics: Vec::new(),
            diagnostic_offset,
        }
    }

    pub(crate) fn process_signal<L: biome_rowan::Language>(
        &mut self,
        signal: &dyn AnalyzerSignal<L>,
    ) -> ControlFlow<Never> {
        let diagnostic = signal.diagnostic();

        if let Some(mut diagnostic) = diagnostic {
            let actions: Vec<_> = signal
                .actions()
                .into_code_action_iter()
                .map(|item| CodeAction {
                    category: item.category.clone(),
                    rule_name: item
                        .rule_name
                        .map(|(group, name)| (Cow::Borrowed(group), Cow::Borrowed(name))),
                    suggestion: item.suggestion,
                    offset: None,
                })
                .collect();
            if !actions.is_empty() {
                if let Some(offset) = &self.diagnostic_offset {
                    diagnostic.add_diagnostic_offset(*offset);
                }
                self.diagnostics.push((
                    biome_diagnostics::serde::Diagnostic::new(Error::from(diagnostic)),
                    actions,
                ));
            }
        }

        ControlFlow::<Never>::Continue(())
    }

    pub(crate) fn finish(self) -> PullDiagnosticsAndActionsResult {
        PullDiagnosticsAndActionsResult {
            diagnostics: self.diagnostics,
        }
    }
}

pub(crate) struct CodeActionsParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) range: Option<TextRange>,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) path: &'a BiomePath,
    pub(crate) module_graph: Arc<ModuleGraph>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) language: DocumentFileSource,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) categories: RuleCategories,
    pub(crate) action_offset: Option<TextSize>,
    pub(crate) document_services: &'a DocumentServices,
}

pub(crate) struct UpdateSnippetsNodes {
    pub(crate) range: TextRange,
    pub(crate) new_code: String,
}

type Lint = fn(LintParams) -> LintResults;
type CodeActions = fn(CodeActionsParams) -> PullActionsResult;
type FixAll = fn(FixAllParams) -> Result<FixFileResult, WorkspaceError>;
type Rename = fn(&BiomePath, AnyParse, TextSize, String) -> Result<RenameResult, WorkspaceError>;
type UpdateSnippets = fn(AnyParse, Vec<UpdateSnippetsNodes>) -> Result<SendNode, WorkspaceError>;
type PullDiagnosticsAndActions = fn(DiagnosticsAndActionsParams) -> PullDiagnosticsAndActionsResult;

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
    /// It updates the snippets contained in the original root
    pub(crate) update_snippets: Option<UpdateSnippets>,
    /// Pulls diagnostics with relative code actions
    pub(crate) pull_diagnostics_and_actions: Option<PullDiagnosticsAndActions>,
}

type Format = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
) -> Result<Printed, WorkspaceError>;
type FormatRange = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
    TextRange,
) -> Result<Printed, WorkspaceError>;
type FormatOnType = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
    TextSize,
) -> Result<Printed, WorkspaceError>;

type FormatEmbedded = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
    Vec<FormatEmbedNode>,
) -> Result<Printed, WorkspaceError>;

#[derive(Debug)]
pub(crate) struct FormatEmbedNode {
    pub(crate) range: TextRange,
    pub(crate) node: AnyParse,
    pub(crate) source: DocumentFileSource,
}

#[derive(Default)]
pub(crate) struct FormatterCapabilities {
    /// It formats a file
    pub(crate) format: Option<Format>,
    /// It formats a portion of text of a file
    pub(crate) format_range: Option<FormatRange>,
    /// It formats a file while typing
    pub(crate) format_on_type: Option<FormatOnType>,
    /// It formats a file with embedded nodes
    pub(crate) format_embedded: Option<FormatEmbedded>,
}

type Enabled = fn(&Utf8Path, &SettingsWithEditor) -> bool;

type Search = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &GritQuery,
    &SettingsWithEditor,
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
    markdown: MarkdownFileHandler,
    ignore: IgnoreFileHandler,
}

impl Features {
    pub(crate) fn new() -> Self {
        Self {
            js: JsFileHandler {},
            json: JsonFileHandler {},
            css: CssFileHandler {},
            astro: AstroFileHandler {},
            svelte: SvelteFileHandler {},
            vue: VueFileHandler {},
            graphql: GraphqlFileHandler {},
            html: HtmlFileHandler {},
            grit: GritFileHandler {},
            markdown: MarkdownFileHandler {},
            ignore: IgnoreFileHandler {},
            unknown: UnknownFileHandler::default(),
        }
    }

    /// Returns the [Capabilities] associated with a [BiomePath]
    pub(crate) fn get_capabilities(&self, language_hint: DocumentFileSource) -> Capabilities {
        match language_hint {
            // TODO: remove match once we remove vue/astro/svelte handlers
            DocumentFileSource::Js(source) => match source.as_embedding_kind() {
                EmbeddingKind::Astro { .. } => self.astro.capabilities(),
                EmbeddingKind::Vue { .. } => self.vue.capabilities(),
                EmbeddingKind::Svelte { .. } => self.svelte.capabilities(),
                EmbeddingKind::None => self.js.capabilities(),
            },
            DocumentFileSource::Json(_) => self.json.capabilities(),
            DocumentFileSource::Css(_) => self.css.capabilities(),
            DocumentFileSource::Graphql(_) => self.graphql.capabilities(),
            DocumentFileSource::Html(_) => self.html.capabilities(),
            DocumentFileSource::Grit(_) => self.grit.capabilities(),
            DocumentFileSource::Markdown(_) => self.markdown.capabilities(),
            DocumentFileSource::Ignore => self.ignore.capabilities(),
            DocumentFileSource::Unknown => self.unknown.capabilities(),
        }
    }
}

/// Checks whether a diagnostic coming from the analyzer is an [error](Severity::Error)
///
/// The function checks the diagnostic against the current configured rules.
// TODO: this function works only with lint rules, but it should work with assist actions too
pub(crate) fn is_diagnostic_error(
    diagnostic: &'_ AnalyzerDiagnostic,
    rules: Option<&'_ Rules>,
) -> bool {
    let severity = diagnostic
        .category()
        .filter(|category| category.name().starts_with("lint/"))
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

#[derive(Default)]
pub struct ParsedLangAndSetup {
    language: Language,
    variant: LanguageVariant,
    setup: bool,
}

fn get_module_item_attributes(item: AnyJsModuleItem) -> Option<JsxAttributeList> {
    let expression = item
        .as_any_js_statement()?
        .as_js_expression_statement()?
        .expression()
        .ok()?;
    let tag = expression.as_jsx_tag_expression()?.tag().ok()?;
    let opening_element = tag.as_jsx_element()?.opening_element().ok()?;
    Some(opening_element.attributes())
}

fn get_attribute_value(attribute: &JsxAttribute) -> Option<TokenText> {
    let attribute_value = attribute.initializer()?.value().ok()?;
    let attribute_inner_string = attribute_value.as_jsx_string()?.inner_string_text().ok()?;
    Some(attribute_inner_string)
}

/// Parse the "lang" and "setup" attributes from the opening tag of the "\<script\>" block in Svelte or Vue files.
/// This function will return the language based on the existence or the value of the "lang" attribute.
/// We use the JSX parser at the moment to parse the opening tag. So the opening tag should be first
/// matched by regular expressions.
///
// TODO: We should change the parser when HTMLish languages are supported.
pub(crate) fn parse_lang_and_setup_from_script_opening_tag(
    script_opening_tag: &str,
) -> ParsedLangAndSetup {
    let Some(tree) = parse(
        script_opening_tag,
        JsFileSource::jsx(),
        JsParserOptions::default(),
    )
    .try_tree() else {
        return ParsedLangAndSetup::default();
    };

    let Some(js_module) = tree.as_js_module() else {
        return ParsedLangAndSetup::default();
    };

    let mut lang_and_setup = ParsedLangAndSetup::default();
    for item in js_module.items() {
        let Some(attributes) = get_module_item_attributes(item) else {
            continue;
        };
        if attributes.find_by_name("setup").is_some() {
            lang_and_setup.setup = true;
        }
        if let Some(lang_attribute) = attributes.find_by_name("lang")
            && let Some(lang_value) = get_attribute_value(&lang_attribute)
        {
            match lang_value.text() {
                "ts" => {
                    lang_and_setup.language = Language::TypeScript {
                        definition_file: false,
                    };
                    lang_and_setup.variant = LanguageVariant::Standard;
                }
                "tsx" => {
                    lang_and_setup.language = Language::TypeScript {
                        definition_file: false,
                    };
                    lang_and_setup.variant = LanguageVariant::Jsx;
                }
                "jsx" => {
                    lang_and_setup.language = Language::JavaScript;
                    lang_and_setup.variant = LanguageVariant::Jsx;
                }
                "js" => {
                    lang_and_setup.language = Language::JavaScript;
                    lang_and_setup.variant = LanguageVariant::Standard;
                }
                _ => {}
            }
        }
    }
    lang_and_setup
}

pub(crate) fn search(
    path: &BiomePath,
    _file_source: &DocumentFileSource,
    parse: AnyParse,
    query: &GritQuery,
    _settings: &SettingsWithEditor,
) -> Result<Vec<TextRange>, WorkspaceError> {
    let result = query
        .execute(GritTargetFile::new(path.as_path(), parse))
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

impl RegistryVisitor<HtmlLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
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
    only: Option<&'b [AnalyzerSelector]>,
    skip: Option<&'b [AnalyzerSelector]>,
    settings: &'b Settings,
    path: Option<&'b Utf8Path>,
    package_json: Option<PackageJson>,
    analyzer_options: &'b mut AnalyzerOptions,
}

impl<'a, 'b> LintVisitor<'a, 'b> {
    pub(crate) fn new(
        only: Option<&'b [AnalyzerSelector]>,
        skip: Option<&'b [AnalyzerSelector]>,
        settings: &'b Settings,
        path: Option<&'b Utf8Path>,
        package_json: Option<PackageJson>,
        analyzer_options: &'b mut AnalyzerOptions,
    ) -> Self {
        Self {
            enabled_rules: Default::default(),
            disabled_rules: Default::default(),
            only,
            skip,
            settings,
            path,
            package_json,
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
        let group = <R::Group as RuleGroup>::NAME;
        // Nursery rules must be enabled only when they are enabled from the group
        if group == "nursery" {
            return;
        }

        let path = self.path.expect("File path");

        let is_recommended = R::METADATA.recommended;
        let recommended_enabled = self.settings.linter_recommended_enabled();
        if !is_recommended || !recommended_enabled {
            return;
        }

        let no_only = self.only.is_some_and(|only| only.is_empty());
        let no_domains = self
            .settings
            .as_linter_domains(path)
            .is_none_or(|d| d.is_empty());
        if !(no_only && no_domains) {
            return;
        }

        if let Some(manifest) = &self.package_json {
            for domain in R::METADATA.domains {
                let matches_a_dependency = domain
                    .manifest_dependencies()
                    .iter()
                    .any(|(dependency, range)| manifest.matches_dependency(dependency, range));

                if matches_a_dependency {
                    self.enabled_rules.insert(rule_filter);

                    self.analyzer_options
                        .push_globals(domain.globals().iter().copied().map(Into::into));
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
        let group = <R::Group as RuleGroup>::NAME;
        // Nursery rules must be enabled only when they are enabled from the group
        if group == "nursery" {
            return;
        }
        if !no_only {
            return;
        }

        let domains = self
            .settings
            .as_linter_domains(self.path.expect("File path"));

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
                            configured_domain.globals().iter().copied().map(Into::into),
                        );
                    }
                    RuleDomainValue::None => {
                        self.disabled_rules.insert(rule_filter);
                    }
                    RuleDomainValue::Recommended => {
                        if R::METADATA.recommended {
                            self.enabled_rules.insert(rule_filter);

                            self.analyzer_options.push_globals(
                                configured_domain.globals().iter().copied().map(Into::into),
                            );
                        }
                    }
                }
            }
        }
    }

    fn finish(mut self) -> (FxHashSet<RuleFilter<'a>>, FxHashSet<RuleFilter<'a>>) {
        let has_only_filter = self.only.is_none_or(|only| !only.is_empty());
        let rules = self
            .settings
            .as_linter_rules(self.path.expect("Path to be set"))
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
        if let Some(rule_filter) = rule_filter
            && rule_filter.match_rule::<R>()
        {
            // first we want to register rules via "magic default"
            self.record_rule_from_manifest::<R, L>(rule_filter);
            // then we want to register rules
            self.record_rule_from_domains::<R, L>(rule_filter);
        };

        // Do not report unused suppression comment diagnostics if:
        // - it is a syntax-only analyzer pass, or
        // - if a single rule is run.
        if let Some(only) = self.only {
            for selector in only {
                match selector {
                    AnalyzerSelector::Rule(selector) => {
                        let filter = RuleFilter::from(selector);
                        if filter.match_rule::<R>() && filter.match_group::<R::Group>() {
                            self.enabled_rules.insert(filter);
                        }
                    }
                    AnalyzerSelector::Domain(selector) => {
                        if selector.match_rule::<R>() {
                            self.enabled_rules.extend(selector.as_rule_filters());
                        }
                    }
                }
            }
        }
        if let Some(skip) = self.skip {
            for selector in skip {
                match selector {
                    AnalyzerSelector::Rule(selector) => {
                        let filter = RuleFilter::from(selector);
                        if filter.match_rule::<R>() && filter.match_group::<R::Group>() {
                            self.disabled_rules.insert(filter);
                        }
                    }
                    AnalyzerSelector::Domain(selector) => {
                        if selector.match_rule::<R>() {
                            self.disabled_rules.extend(selector.as_rule_filters());
                        }
                    }
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

impl RegistryVisitor<HtmlLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = HtmlLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            biome_html_analyze::METADATA
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

struct AssistsVisitor<'a, 'b> {
    settings: &'b Settings,
    enabled_rules: Vec<RuleFilter<'a>>,
    disabled_rules: Vec<RuleFilter<'a>>,
    only: Option<&'b [AnalyzerSelector]>,
    skip: Option<&'b [AnalyzerSelector]>,
    path: Option<&'b Utf8Path>,
}

impl<'a, 'b> AssistsVisitor<'a, 'b> {
    pub(crate) fn new(
        only: Option<&'b [AnalyzerSelector]>,
        skip: Option<&'b [AnalyzerSelector]>,
        settings: &'b Settings,
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
                match selector {
                    AnalyzerSelector::Rule(rule) => {
                        let filter = RuleFilter::from(rule);
                        if filter.match_rule::<R>() {
                            self.enabled_rules.push(filter)
                        }
                    }
                    AnalyzerSelector::Domain(domain) => {
                        if domain.match_rule::<R>() {
                            self.enabled_rules.extend(domain.as_rule_filters());
                        }
                    }
                }
            }
        }

        if let Some(skip) = self.skip {
            for selector in skip {
                match selector {
                    AnalyzerSelector::Rule(rule) => {
                        let filter = RuleFilter::from(rule);
                        if filter.match_rule::<R>() {
                            self.disabled_rules.push(filter)
                        }
                    }
                    AnalyzerSelector::Domain(domain) => {
                        if domain.match_rule::<R>() {
                            self.disabled_rules.extend(domain.as_rule_filters());
                        }
                    }
                }
            }
        }
    }

    fn finish(mut self) -> (Vec<RuleFilter<'a>>, Vec<RuleFilter<'a>>) {
        let has_only_filter = self.only.is_none_or(|only| !only.is_empty());
        let rules = self
            .settings
            .as_assist_actions(self.path.expect("Path to be set"))
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

impl RegistryVisitor<HtmlLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

pub(crate) struct AnalyzerVisitorBuilder<'a> {
    settings: &'a Settings,
    only: Option<&'a [AnalyzerSelector]>,
    skip: Option<&'a [AnalyzerSelector]>,
    path: Option<&'a Utf8Path>,
    enabled_selectors: Option<&'a [AnalyzerSelector]>,
    project_layout: Arc<ProjectLayout>,
    analyzer_options: AnalyzerOptions,
}

impl<'b> AnalyzerVisitorBuilder<'b> {
    pub(crate) fn new(settings: &'b Settings, analyzer_options: AnalyzerOptions) -> Self {
        Self {
            settings,
            only: None,
            skip: None,
            path: None,
            enabled_selectors: None,
            project_layout: Default::default(),
            analyzer_options,
        }
    }

    #[must_use]
    pub(crate) fn with_only(mut self, only: &'b [AnalyzerSelector]) -> Self {
        self.only = Some(only);
        self
    }

    #[must_use]
    pub(crate) fn with_skip(mut self, skip: &'b [AnalyzerSelector]) -> Self {
        self.skip = Some(skip);
        self
    }

    #[must_use]
    pub(crate) fn with_path(mut self, path: &'b Utf8Path) -> Self {
        self.path = Some(path);
        self
    }

    #[must_use]
    pub(crate) fn with_enabled_selectors(mut self, enabled_rules: &'b [AnalyzerSelector]) -> Self {
        self.enabled_selectors = Some(enabled_rules);
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
        let mut enabled_rules = vec![];

        if let Some(enabled_selectors) = self.enabled_selectors {
            for selector in enabled_selectors {
                match selector {
                    AnalyzerSelector::Rule(rule) => {
                        enabled_rules.push(RuleFilter::from(rule));
                    }
                    AnalyzerSelector::Domain(domain) => {
                        enabled_rules.extend(domain.as_rule_filters())
                    }
                }
            }
        }
        let mut syntax = SyntaxVisitor::default();

        biome_js_analyze::visit_registry(&mut syntax);
        biome_css_analyze::visit_registry(&mut syntax);
        biome_json_analyze::visit_registry(&mut syntax);
        biome_graphql_analyze::visit_registry(&mut syntax);
        biome_html_analyze::visit_registry(&mut syntax);
        enabled_rules.extend(syntax.enabled_rules);

        let package_json = self
            .path
            .and_then(|path| self.project_layout.find_node_manifest_for_path(path))
            .map(|(_, manifest)| manifest);

        // Query tsconfig.json for JSX factory settings if jsx_runtime is ReactClassic
        // and the factory settings are not already set
        if let Some(path) = self.path
            && analyzer_options.jsx_runtime()
                == Some(biome_analyze::options::JsxRuntime::ReactClassic)
        {
            if analyzer_options.jsx_factory().is_none() {
                let factory = self
                    .project_layout
                    .query_tsconfig_for_path(path, |tsconfig| {
                        tsconfig.jsx_factory_identifier().map(|s| s.to_string())
                    })
                    .flatten();
                analyzer_options.set_jsx_factory(factory.map(|s| s.into()));
            }
            if analyzer_options.jsx_fragment_factory().is_none() {
                let fragment_factory = self
                    .project_layout
                    .query_tsconfig_for_path(path, |tsconfig| {
                        tsconfig
                            .jsx_fragment_factory_identifier()
                            .map(|s| s.to_string())
                    })
                    .flatten();
                analyzer_options.set_jsx_fragment_factory(fragment_factory.map(|s| s.into()));
            }
        }

        let mut lint = LintVisitor::new(
            self.only,
            self.skip,
            self.settings,
            self.path,
            package_json,
            &mut analyzer_options,
        );

        biome_js_analyze::visit_registry(&mut lint);
        biome_css_analyze::visit_registry(&mut lint);
        biome_json_analyze::visit_registry(&mut lint);
        biome_graphql_analyze::visit_registry(&mut lint);
        biome_html_analyze::visit_registry(&mut lint);
        let (linter_enabled_rules, linter_disabled_rules) = lint.finish();
        enabled_rules.extend(linter_enabled_rules);
        disabled_rules.extend(linter_disabled_rules);

        let mut assist = AssistsVisitor::new(self.only, self.skip, self.settings, self.path);

        biome_js_analyze::visit_registry(&mut assist);
        biome_css_analyze::visit_registry(&mut assist);
        biome_json_analyze::visit_registry(&mut assist);
        biome_graphql_analyze::visit_registry(&mut assist);
        biome_html_analyze::visit_registry(&mut assist);
        let (assists_enabled_rules, assists_disabled_rules) = assist.finish();
        enabled_rules.extend(assists_enabled_rules);
        disabled_rules.extend(assists_disabled_rules);

        (enabled_rules, disabled_rules, analyzer_options)
    }
}

#[cfg(test)]
mod tests {
    use super::{DocumentFileSource, Features};
    use camino::Utf8Path;

    #[test]
    fn markdown_file_source_detection_and_capabilities() {
        let source = DocumentFileSource::from_path(Utf8Path::new("docs/readme.md"), false);
        assert!(matches!(source, DocumentFileSource::Unknown));

        let language_source = DocumentFileSource::from_language_id("markdown");
        assert!(matches!(language_source, DocumentFileSource::Unknown));

        assert!(!DocumentFileSource::can_parse(Utf8Path::new(
            "docs/readme.md"
        )));
        assert!(!DocumentFileSource::can_read(Utf8Path::new(
            "docs/readme.md"
        )));
        assert!(!DocumentFileSource::can_contain_embeds(
            Utf8Path::new("docs/readme.md"),
            false
        ));
    }

    #[test]
    fn markdown_features_provide_formatter_capabilities() {
        let features = Features::new();
        let capabilities = features.get_capabilities(DocumentFileSource::from_path(
            Utf8Path::new("doc.md"),
            false,
        ));

        assert!(capabilities.formatter.format.is_none());
        assert!(capabilities.parser.parse.is_none());
    }
}
