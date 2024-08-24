use self::{
    css::CssFileHandler, javascript::JsFileHandler, json::JsonFileHandler,
    unknown::UnknownFileHandler,
};
use crate::diagnostics::{QueryDiagnostic, SearchError};
pub use crate::file_handlers::astro::{AstroFileHandler, ASTRO_FENCE};
use crate::file_handlers::graphql::GraphqlFileHandler;
pub use crate::file_handlers::svelte::{SvelteFileHandler, SVELTE_FENCE};
pub use crate::file_handlers::vue::{VueFileHandler, VUE_FENCE};
use crate::settings::Settings;
use crate::workspace::{FixFileMode, OrganizeImportsResult};
use crate::{
    settings::WorkspaceSettingsHandle,
    workspace::{FixFileResult, GetSyntaxTreeResult, PullActionsResult, RenameResult},
    WorkspaceError,
};
use biome_analyze::{
    AnalyzerDiagnostic, GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategories,
    RuleCategory, RuleFilter, RuleGroup,
};
use biome_configuration::analyzer::RuleSelector;
use biome_configuration::Rules;
use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_css_syntax::{CssFileSource, CssLanguage};
use biome_diagnostics::{Diagnostic, Severity};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use biome_grit_patterns::{GritQuery, GritQueryResult, GritTargetFile};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage, Language, TextRange, TextSize};
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_parser::AnyParse;
use biome_project::PackageJson;
use biome_rowan::{FileSourceError, NodeCache};
use biome_string_case::StrExtension;
pub use javascript::JsFormatterSettings;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;

mod astro;
mod css;
mod graphql;
mod javascript;
mod json;
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

impl From<&Path> for DocumentFileSource {
    fn from(path: &Path) -> Self {
        Self::from_path(path)
    }
}

impl DocumentFileSource {
    fn try_from_well_known(path: &Path) -> Result<Self, FileSourceError> {
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
    pub fn from_well_known(path: &Path) -> Self {
        Self::try_from_well_known(path)
            .map_or(DocumentFileSource::Unknown, |file_source| file_source)
    }

    fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
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
        Err(FileSourceError::UnknownExtension)
    }

    /// Returns the document file source corresponding to this file extension
    pub fn from_extension(extension: impl AsRef<OsStr>) -> Self {
        Self::try_from_extension(extension.as_ref())
            .map_or(DocumentFileSource::Unknown, |file_source| file_source)
    }

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
        Err(FileSourceError::UnknownLanguageId)
    }

    /// Returns the document file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn from_language_id(language_id: &str) -> Self {
        Self::try_from_language_id(language_id)
            .map_or(DocumentFileSource::Unknown, |file_source| file_source)
    }

    fn try_from_path(path: &Path) -> Result<Self, FileSourceError> {
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
            Some(filename) if filename.as_encoded_bytes().ends_with(b".d.ts") => {
                Cow::Borrowed("d.ts".as_ref())
            }
            Some(filename) if filename.as_encoded_bytes().ends_with(b".d.mts") => {
                Cow::Borrowed("d.mts".as_ref())
            }
            Some(filename) if filename.as_encoded_bytes().ends_with(b".d.cts") => {
                Cow::Borrowed("d.cts".as_ref())
            }
            _ => path
                .extension()
                // We assume the file extensions are case-insensitive.
                // Thus, we normalize the extension to lowercase.
                .map(|ext| ext.to_ascii_lowercase_cow())
                .ok_or(FileSourceError::MissingFileExtension)?,
        };

        Self::try_from_extension(extension)
    }

    /// Returns the document file source corresponding to the file path
    pub fn from_path(path: &Path) -> Self {
        Self::try_from_path(path).map_or(DocumentFileSource::Unknown, |file_source| file_source)
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

    pub fn to_css_file_source(&self) -> Option<CssFileSource> {
        match self {
            DocumentFileSource::Css(css) => Some(*css),
            _ => None,
        }
    }

    pub fn can_parse(path: &Path, content: &str) -> bool {
        let file_source = DocumentFileSource::from(path);
        match file_source {
            DocumentFileSource::Js(js) => match js.as_embedding_kind() {
                EmbeddingKind::Astro => ASTRO_FENCE.is_match(content),
                EmbeddingKind::Vue => VUE_FENCE.is_match(content),
                EmbeddingKind::Svelte => SVELTE_FENCE.is_match(content),
                EmbeddingKind::None => true,
            },
            DocumentFileSource::Json(_) | DocumentFileSource::Css(_) => true,
            DocumentFileSource::Graphql(_) => true,
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
            DocumentFileSource::Unknown => fmt.write_markup(markup! { "Unknown" }),
        }
    }
}

pub struct FixAllParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) workspace: WorkspaceSettingsHandle<'a>,
    /// Whether it should format the code action
    pub(crate) should_format: bool,
    pub(crate) biome_path: &'a BiomePath,
    pub(crate) manifest: Option<PackageJson>,
    pub(crate) document_file_source: DocumentFileSource,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) rule_categories: RuleCategories,
}

#[derive(Default)]
/// The list of capabilities that are available for a language
pub struct Capabilities {
    pub(crate) parser: ParserCapabilities,
    pub(crate) debug: DebugCapabilities,
    pub(crate) analyzer: AnalyzerCapabilities,
    pub(crate) formatter: FormatterCapabilities,
    pub(crate) search: SearchCapabilities,
}

#[derive(Clone)]
pub struct ParseResult {
    pub(crate) any_parse: AnyParse,
    pub(crate) language: Option<DocumentFileSource>,
}

type Parse =
    fn(&BiomePath, DocumentFileSource, &str, Option<&Settings>, &mut NodeCache) -> ParseResult;

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
    pub(crate) workspace: &'a WorkspaceSettingsHandle<'a>,
    pub(crate) language: DocumentFileSource,
    pub(crate) max_diagnostics: u32,
    pub(crate) path: &'a BiomePath,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) categories: RuleCategories,
    pub(crate) manifest: Option<PackageJson>,
}

pub(crate) struct LintResults {
    pub(crate) diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    pub(crate) errors: usize,
    pub(crate) skipped_diagnostics: u32,
}

pub(crate) struct CodeActionsParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) range: Option<TextRange>,
    pub(crate) workspace: &'a WorkspaceSettingsHandle<'a>,
    pub(crate) path: &'a BiomePath,
    pub(crate) manifest: Option<PackageJson>,
    pub(crate) language: DocumentFileSource,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
}

type Lint = fn(LintParams) -> LintResults;
type CodeActions = fn(CodeActionsParams) -> PullActionsResult;
type FixAll = fn(FixAllParams) -> Result<FixFileResult, WorkspaceError>;
type Rename = fn(&BiomePath, AnyParse, TextSize, String) -> Result<RenameResult, WorkspaceError>;
type OrganizeImports = fn(AnyParse) -> Result<OrganizeImportsResult, WorkspaceError>;

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
    /// It organizes imports
    pub(crate) organize_imports: Option<OrganizeImports>,
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
    #[allow(unused)]
    css: CssFileHandler,
    astro: AstroFileHandler,
    vue: VueFileHandler,
    svelte: SvelteFileHandler,
    unknown: UnknownFileHandler,
    graphql: GraphqlFileHandler,
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
            unknown: UnknownFileHandler::default(),
        }
    }

    /// Returns the [Capabilities] associated with a [BiomePath]
    pub(crate) fn get_capabilities(
        &self,
        biome_path: &BiomePath,
        language_hint: DocumentFileSource,
    ) -> Capabilities {
        match DocumentFileSource::from_path(biome_path).or(language_hint) {
            DocumentFileSource::Js(source) => match source.as_embedding_kind() {
                EmbeddingKind::Astro => self.astro.capabilities(),
                EmbeddingKind::Vue => self.vue.capabilities(),
                EmbeddingKind::Svelte => self.svelte.capabilities(),
                EmbeddingKind::None => self.js.capabilities(),
            },
            DocumentFileSource::Json(_) => self.json.capabilities(),
            DocumentFileSource::Css(_) => self.css.capabilities(),
            DocumentFileSource::Graphql(_) => self.graphql.capabilities(),
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
        .filter(|category| category.name().starts_with("lint/"))
        .map_or_else(
            || diagnostic.severity(),
            |category| {
                rules
                    .and_then(|rules| rules.get_severity_from_code(category))
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
pub(crate) fn parse_lang_from_script_opening_tag(script_opening_tag: &str) -> Language {
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
            let lang_attribute = opening_element.attributes().find_by_name("lang").ok()??;
            let attribute_value = lang_attribute.initializer()?.value().ok()?;
            let attribute_inner_string =
                attribute_value.as_jsx_string()?.inner_string_text().ok()?;
            match attribute_inner_string.text() {
                "ts" | "tsx" => Some(Language::TypeScript {
                    definition_file: false,
                }),
                _ => None,
            }
        })
    })
    .map_or(Language::JavaScript, |lang| lang)
}

pub(crate) fn search(
    path: &BiomePath,
    _file_source: &DocumentFileSource,
    parse: AnyParse,
    query: &GritQuery,
    _settings: WorkspaceSettingsHandle,
) -> Result<Vec<TextRange>, WorkspaceError> {
    let query_result = query
        .execute(GritTargetFile {
            path: path.to_path_buf(),
            parse,
        })
        .map_err(|err| {
            WorkspaceError::SearchError(SearchError::QueryError(QueryDiagnostic(err.to_string())))
        })?;

    let matches = query_result
        .into_iter()
        .flat_map(|result| match result {
            GritQueryResult::Match(m) => m.ranges,
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

    assert!(parse_lang_from_script_opening_tag(SVELTE_JS_SCRIPT_OPENING_TAG).is_javascript());
    assert!(parse_lang_from_script_opening_tag(SVELTE_TS_SCRIPT_OPENING_TAG).is_typescript());
    assert!(
        parse_lang_from_script_opening_tag(SVELTE_CONTEXT_MODULE_JS_SCRIPT_OPENING_TAG)
            .is_javascript()
    );
    assert!(
        parse_lang_from_script_opening_tag(SVELTE_CONTEXT_MODULE_TS_SCRIPT_OPENING_TAG)
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

impl<'a> RegistryVisitor<JsLanguage> for SyntaxVisitor<'a> {
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

impl<'a> RegistryVisitor<JsonLanguage> for SyntaxVisitor<'a> {
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

impl<'a> RegistryVisitor<CssLanguage> for SyntaxVisitor<'a> {
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

impl<'a> RegistryVisitor<GraphqlLanguage> for SyntaxVisitor<'a> {
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
    pub(crate) enabled_rules: Vec<RuleFilter<'a>>,
    pub(crate) disabled_rules: Vec<RuleFilter<'a>>,
    // lint_params: &'b LintParams<'a>,
    only: &'b Vec<RuleSelector>,
    skip: &'b Vec<RuleSelector>,
    settings: Option<&'b Settings>,
    path: &'b Path,
}

impl<'a, 'b> LintVisitor<'a, 'b> {
    pub(crate) fn new(
        only: &'b Vec<RuleSelector>,
        skip: &'b Vec<RuleSelector>,
        settings: Option<&'b Settings>,
        path: &'b Path,
    ) -> Self {
        Self {
            enabled_rules: vec![],
            disabled_rules: vec![],
            only,
            skip,
            settings,
            path,
        }
    }

    fn finish(mut self) -> (Vec<RuleFilter<'a>>, Vec<RuleFilter<'a>>) {
        let has_only_filter = !self.only.is_empty();
        let enabled_rules = if !has_only_filter {
            self.settings
                .and_then(|settings| settings.as_linter_rules(self.path))
                .as_ref()
                .map(|rules| rules.as_enabled_rules())
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>()
        } else {
            vec![]
        };
        self.enabled_rules.extend(enabled_rules);
        (self.enabled_rules, self.disabled_rules)
    }

    fn push_rule<R, L>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        // Do not report unused suppression comment diagnostics if:
        // - it is a syntax-only analyzer pass, or
        // - if a single rule is run.
        for selector in self.only {
            let filter = RuleFilter::from(selector);
            if filter.match_rule::<R>() {
                self.enabled_rules.push(filter)
            }
        }
        for selector in self.skip {
            let filter = RuleFilter::from(selector);
            if filter.match_rule::<R>() {
                self.disabled_rules.push(filter)
            }
        }
    }
}

impl<'a, 'b> RegistryVisitor<JsLanguage> for LintVisitor<'a, 'b> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = JsLanguage>>(&mut self) {
        for selector in self.only {
            if RuleFilter::from(selector).match_group::<G>() {
                G::record_rules(self)
            }
        }

        for selector in self.skip {
            if RuleFilter::from(selector).match_group::<G>() {
                G::record_rules(self)
            }
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>()
    }
}
impl<'a, 'b> RegistryVisitor<JsonLanguage> for LintVisitor<'a, 'b> {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = JsonLanguage>>(&mut self) {
        for selector in self.only {
            if RuleFilter::from(selector).match_group::<G>() {
                G::record_rules(self)
            }
        }

        for selector in self.skip {
            if RuleFilter::from(selector).match_group::<G>() {
                G::record_rules(self)
            }
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>()
    }
}

impl<'a, 'b> RegistryVisitor<CssLanguage> for LintVisitor<'a, 'b> {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = CssLanguage>>(&mut self) {
        for selector in self.only {
            if RuleFilter::from(selector).match_group::<G>() {
                G::record_rules(self)
            }
        }

        for selector in self.skip {
            if RuleFilter::from(selector).match_group::<G>() {
                G::record_rules(self)
            }
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>()
    }
}

impl<'a, 'b> RegistryVisitor<GraphqlLanguage> for LintVisitor<'a, 'b> {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = GraphqlLanguage>>(&mut self) {
        for selector in self.only {
            if RuleFilter::from(selector).match_group::<G>() {
                G::record_rules(self)
            }
        }

        for selector in self.skip {
            if RuleFilter::from(selector).match_group::<G>() {
                G::record_rules(self)
            }
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>()
    }
}

struct AssistsVisitor<'a, 'b> {
    settings: Option<&'b Settings>,
    enabled_rules: Vec<RuleFilter<'a>>,
    disabled_rules: Vec<RuleFilter<'a>>,
    import_sorting: RuleFilter<'a>,
    path: &'b Path,
    only: &'b Vec<RuleSelector>,
    skip: &'b Vec<RuleSelector>,
}

impl<'a, 'b> AssistsVisitor<'a, 'b> {
    pub(crate) fn new(
        only: &'b Vec<RuleSelector>,
        skip: &'b Vec<RuleSelector>,
        settings: Option<&'b Settings>,
        path: &'b Path,
    ) -> Self {
        Self {
            enabled_rules: vec![],
            disabled_rules: vec![],
            settings,
            import_sorting: RuleFilter::Rule("source", "organizeImports"),
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

        let organize_imports_enabled = self
            .settings
            .map(|settings| settings.organize_imports.enabled)
            .unwrap_or_default();
        if organize_imports_enabled && self.import_sorting.match_rule::<R>() {
            self.enabled_rules.push(self.import_sorting);
            return;
        }
        // Do not report unused suppression comment diagnostics if:
        // - it is a syntax-only analyzer pass, or
        // - if a single rule is run.
        for selector in self.only {
            let filter = RuleFilter::from(selector);
            if filter.match_rule::<R>() {
                self.enabled_rules.push(filter)
            }
        }
        for selector in self.skip {
            let filter = RuleFilter::from(selector);
            if filter.match_rule::<R>() {
                self.disabled_rules.push(filter)
            }
        }
    }

    fn finish(mut self) -> (Vec<RuleFilter<'a>>, Vec<RuleFilter<'a>>) {
        let enabled_rules = self
            .settings
            .and_then(|settings| settings.as_assists_rules(self.path))
            .as_ref()
            .map(|rules| rules.as_enabled_rules())
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>();
        self.enabled_rules.extend(enabled_rules);
        (self.enabled_rules, self.disabled_rules)
    }
}

impl<'a, 'b> RegistryVisitor<JsLanguage> for AssistsVisitor<'a, 'b> {
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

impl<'a, 'b> RegistryVisitor<JsonLanguage> for AssistsVisitor<'a, 'b> {
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

impl<'a, 'b> RegistryVisitor<CssLanguage> for AssistsVisitor<'a, 'b> {
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

impl<'a, 'b> RegistryVisitor<GraphqlLanguage> for AssistsVisitor<'a, 'b> {
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

pub(crate) struct AnalyzerVisitorBuilder<'a, 'b> {
    syntax: Option<SyntaxVisitor<'a>>,
    lint: Option<LintVisitor<'a, 'b>>,
    assists: Option<AssistsVisitor<'a, 'b>>,
    settings: Option<&'b Settings>,
}

impl<'a, 'b> AnalyzerVisitorBuilder<'a, 'b> {
    pub(crate) fn new(settings: Option<&'b Settings>) -> Self {
        Self {
            settings,
            syntax: None,
            lint: None,
            assists: None,
        }
    }

    #[must_use]
    pub(crate) fn with_syntax_rules(mut self) -> Self {
        self.syntax = Some(SyntaxVisitor::default());
        self
    }
    #[must_use]
    pub(crate) fn with_linter_rules(
        mut self,
        only: &'b Vec<RuleSelector>,
        skip: &'b Vec<RuleSelector>,
        path: &'b Path,
    ) -> Self {
        self.lint = Some(LintVisitor::new(only, skip, self.settings, path));
        self
    }

    #[must_use]
    pub(crate) fn with_assists_rules(
        mut self,
        only: &'b Vec<RuleSelector>,
        skip: &'b Vec<RuleSelector>,
        path: &'b Path,
    ) -> Self {
        self.assists = Some(AssistsVisitor::new(only, skip, self.settings, path));
        self
    }

    #[must_use]
    pub(crate) fn finish(self) -> (Vec<RuleFilter<'a>>, Vec<RuleFilter<'a>>) {
        let mut disabled_rules = vec![];
        let mut enabled_rules = vec![];
        if let Some(mut syntax) = self.syntax {
            biome_js_analyze::visit_registry(&mut syntax);
            biome_css_analyze::visit_registry(&mut syntax);
            biome_json_analyze::visit_registry(&mut syntax);
            biome_graphql_analyze::visit_registry(&mut syntax);
            enabled_rules.extend(syntax.enabled_rules);
        }

        if let Some(mut lint) = self.lint {
            biome_js_analyze::visit_registry(&mut lint);
            biome_css_analyze::visit_registry(&mut lint);
            biome_json_analyze::visit_registry(&mut lint);
            biome_graphql_analyze::visit_registry(&mut lint);
            let (linter_enabled_rules, linter_disabled_rules) = lint.finish();
            enabled_rules.extend(linter_enabled_rules);
            disabled_rules.extend(linter_disabled_rules);
        }

        if let Some(mut assists) = self.assists {
            biome_js_analyze::visit_registry(&mut assists);
            biome_css_analyze::visit_registry(&mut assists);
            biome_json_analyze::visit_registry(&mut assists);
            biome_graphql_analyze::visit_registry(&mut assists);
            let (assists_enabled_rules, assists_disabled_rules) = assists.finish();
            enabled_rules.extend(assists_enabled_rules);
            disabled_rules.extend(assists_disabled_rules);
        }

        (enabled_rules, disabled_rules)
    }
}

#[test]
fn test_vue_script_lang() {
    const VUE_JS_SCRIPT_OPENING_TAG: &str = r#"<script>"#;
    const VUE_TS_SCRIPT_OPENING_TAG: &str = r#"<script lang="ts">"#;
    const VUE_TSX_SCRIPT_OPENING_TAG: &str = r#"<script lang="tsx">"#;
    const VUE_SETUP_JS_SCRIPT_OPENING_TAG: &str = r#"<script setup>"#;
    const VUE_SETUP_TS_SCRIPT_OPENING_TAG: &str = r#"<script setup lang="ts">"#;

    assert!(parse_lang_from_script_opening_tag(VUE_JS_SCRIPT_OPENING_TAG).is_javascript());
    assert!(parse_lang_from_script_opening_tag(VUE_TS_SCRIPT_OPENING_TAG).is_typescript());
    assert!(parse_lang_from_script_opening_tag(VUE_TSX_SCRIPT_OPENING_TAG).is_typescript());
    assert!(parse_lang_from_script_opening_tag(VUE_SETUP_JS_SCRIPT_OPENING_TAG).is_javascript());
    assert!(parse_lang_from_script_opening_tag(VUE_SETUP_TS_SCRIPT_OPENING_TAG).is_typescript());
}
