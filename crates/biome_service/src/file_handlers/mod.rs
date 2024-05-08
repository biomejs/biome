use self::{
    css::CssFileHandler, javascript::JsFileHandler, json::JsonFileHandler,
    unknown::UnknownFileHandler,
};
pub use crate::file_handlers::astro::{AstroFileHandler, ASTRO_FENCE};
pub use crate::file_handlers::svelte::{SvelteFileHandler, SVELTE_FENCE};
pub use crate::file_handlers::vue::{VueFileHandler, VUE_FENCE};
use crate::workspace::{FixFileMode, OrganizeImportsResult};
use crate::{
    settings::WorkspaceSettingsHandle,
    workspace::{FixFileResult, GetSyntaxTreeResult, PullActionsResult, RenameResult},
    WorkspaceError,
};
use biome_analyze::{AnalysisFilter, AnalyzerDiagnostic, RuleCategories};
use biome_configuration::Rules;
use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_css_formatter::can_format_css_yet;
use biome_css_syntax::CssFileSource;
use biome_diagnostics::{Diagnostic, Severity};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{EmbeddingKind, JsFileSource, Language, TextRange, TextSize};
use biome_json_syntax::JsonFileSource;
use biome_parser::AnyParse;
use biome_project::PackageJson;
use biome_rowan::{FileSourceError, NodeCache};
pub use javascript::JsFormatterSettings;
use std::ffi::OsStr;
use std::path::Path;

mod astro;
mod css;
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

impl From<&Path> for DocumentFileSource {
    fn from(path: &Path) -> Self {
        Self::from_path(path)
    }
}

impl DocumentFileSource {
    fn try_from_well_known(file_name: &str) -> Result<Self, FileSourceError> {
        if let Ok(file_source) = JsonFileSource::try_from_well_known(file_name) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = JsFileSource::try_from_well_known(file_name) {
            return Ok(file_source.into());
        }
        if let Ok(file_source) = CssFileSource::try_from_well_known(file_name) {
            return Ok(file_source.into());
        }
        Err(FileSourceError::UnknownFileName(file_name.into()))
    }

    /// Returns the document file source corresponding to this file name from well-known files
    pub fn from_well_known(file_name: &str) -> Self {
        Self::try_from_well_known(file_name)
            .map_or(DocumentFileSource::Unknown, |file_source| file_source)
    }

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
        Err(FileSourceError::UnknownExtension(
            Default::default(),
            extension.into(),
        ))
    }

    /// Returns the document file source corresponding to this file extension
    pub fn from_extension(extension: &str) -> Self {
        Self::try_from_extension(extension)
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
        Err(FileSourceError::UnknownLanguageId(language_id.into()))
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
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?;

        if let Ok(file_source) = Self::try_from_well_known(file_name) {
            return Ok(file_source);
        }

        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        // TODO: This should be extracted to a dedicated function, maybe in biome_fs
        // because the same logic is also used in JsFileSource::try_from
        // and we may support more and more extensions with more than one dots.
        let extension = &match path {
            _ if path
                .to_str()
                .is_some_and(|p| p.to_lowercase().ends_with(".d.ts")) =>
            {
                Some("d.ts".to_owned())
            }
            _ if path
                .to_str()
                .is_some_and(|p| p.to_lowercase().ends_with(".d.mts")) =>
            {
                Some("d.mts".to_owned())
            }
            _ if path
                .to_str()
                .is_some_and(|p| p.to_lowercase().ends_with(".d.cts")) =>
            {
                Some("d.cts".to_owned())
            }
            path => path
                .extension()
                .and_then(OsStr::to_str)
                .map(|s| s.to_lowercase()),
        }
        .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?;

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
            DocumentFileSource::Unknown => fmt.write_markup(markup! { "Unknown" }),
        }
    }
}

pub struct FixAllParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) rules: Option<&'a Rules>,
    pub(crate) filter: AnalysisFilter<'a>,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) settings: WorkspaceSettingsHandle<'a>,
    /// Whether it should format the code action
    pub(crate) should_format: bool,
    pub(crate) biome_path: &'a BiomePath,
    pub(crate) manifest: Option<PackageJson>,
    pub(crate) document_file_source: DocumentFileSource,
}

#[derive(Default)]
/// The list of capabilities that are available for a language
pub struct Capabilities {
    pub(crate) parser: ParserCapabilities,
    pub(crate) debug: DebugCapabilities,
    pub(crate) analyzer: AnalyzerCapabilities,
    pub(crate) formatter: FormatterCapabilities,
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

pub(crate) struct LintParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) settings: WorkspaceSettingsHandle<'a>,
    pub(crate) language: DocumentFileSource,
    pub(crate) max_diagnostics: u32,
    pub(crate) path: &'a BiomePath,
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
    pub(crate) range: TextRange,
    pub(crate) workspace: WorkspaceSettingsHandle<'a>,
    pub(crate) path: &'a BiomePath,
    pub(crate) manifest: Option<PackageJson>,
    pub(crate) language: DocumentFileSource,
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
    /// It organize imports
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
            DocumentFileSource::Css(_) => {
                // TODO: change this when we are ready to handle CSS files
                if can_format_css_yet() {
                    self.css.capabilities()
                } else {
                    self.unknown.capabilities()
                }
            }
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
            if attribute_inner_string.text() == "ts" {
                Some(Language::TypeScript {
                    definition_file: false,
                })
            } else {
                None
            }
        })
    })
    .map_or(Language::JavaScript, |lang| lang)
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

#[test]
fn test_vue_script_lang() {
    const VUE_JS_SCRIPT_OPENING_TAG: &str = r#"<script>"#;
    const VUE_TS_SCRIPT_OPENING_TAG: &str = r#"<script lang="ts">"#;
    const VUE_SETUP_JS_SCRIPT_OPENING_TAG: &str = r#"<script setup>"#;
    const VUE_SETUP_TS_SCRIPT_OPENING_TAG: &str = r#"<script setup lang="ts">"#;

    assert!(parse_lang_from_script_opening_tag(VUE_JS_SCRIPT_OPENING_TAG).is_javascript());
    assert!(parse_lang_from_script_opening_tag(VUE_TS_SCRIPT_OPENING_TAG).is_typescript());
    assert!(parse_lang_from_script_opening_tag(VUE_SETUP_JS_SCRIPT_OPENING_TAG).is_javascript());
    assert!(parse_lang_from_script_opening_tag(VUE_SETUP_TS_SCRIPT_OPENING_TAG).is_typescript());
}
