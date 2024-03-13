use self::{
    css::CssFileHandler, javascript::JsFileHandler, json::JsonFileHandler,
    unknown::UnknownFileHandler,
};
pub use crate::file_handlers::astro::{AstroFileHandler, ASTRO_FENCE};
pub use crate::file_handlers::svelte::{SvelteFileHandler, SVELTE_FENCE};
pub use crate::file_handlers::vue::{VueFileHandler, VUE_FENCE};
use crate::workspace::{FixFileMode, OrganizeImportsResult};
use crate::{
    settings::SettingsHandle,
    workspace::{FixFileResult, GetSyntaxTreeResult, PullActionsResult, RenameResult},
    Rules, WorkspaceError,
};
use biome_analyze::{AnalysisFilter, AnalyzerDiagnostic, RuleCategories};
use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_css_formatter::can_format_css_yet;
use biome_css_syntax::CssFileSource;
use biome_diagnostics::{Diagnostic, Severity};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_syntax::{EmbeddingKind, JsFileSource, ModuleKind, TextRange, TextSize};
use biome_json_syntax::JsonFileSource;
use biome_parser::AnyParse;
use biome_project::PackageJson;
use biome_rowan::NodeCache;
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
    fn from(value: &Path) -> Self {
        JsFileSource::try_from(value)
            .map(Into::into)
            .or(JsonFileSource::try_from(value).map(Into::into))
            .unwrap_or(DocumentFileSource::Unknown)
    }
}

impl DocumentFileSource {
    /// Sorted array of files that are known as JSONC (JSON with comments).
    pub(crate) const KNOWN_FILES_AS_JSONC: &'static [&'static str; 15] = &[
        ".babelrc",
        ".babelrc.json",
        ".ember-cli",
        ".eslintrc",
        ".eslintrc.json",
        ".hintrc",
        ".jsfmtrc",
        ".jshintrc",
        ".swcrc",
        "babel.config.json",
        "jsconfig.json",
        "tsconfig.json",
        "tslint.json",
        "typedoc.json",
        "typescript.json",
    ];

    /// Returns the language corresponding to this language ID
    ///
    /// See the [microsoft spec]
    /// for a list of language identifiers
    ///
    /// [microsoft spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn from_extension(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "js" | "mjs" => JsFileSource::jsx().into(),
            "cjs" => JsFileSource::js_script().into(),
            "jsx" => JsFileSource::jsx().into(),
            "ts" | "mts" => JsFileSource::ts().into(),
            "cts" => JsFileSource::ts()
                .with_module_kind(ModuleKind::Script)
                .into(),
            "d.ts" | "d.mts" | "d.cts" => JsFileSource::d_ts().into(),
            "tsx" => JsFileSource::tsx().into(),
            "json" => JsonFileSource::json().into(),
            "jsonc" => JsonFileSource::jsonc().into(),
            "astro" => JsFileSource::astro().into(),
            "vue" => JsFileSource::vue().into(),
            "svelte" => JsFileSource::svelte().into(),
            "css" => CssFileSource::css().into(),
            _ => DocumentFileSource::Unknown,
        }
    }

    /// Returns the language corresponding to this language ID
    ///
    /// See the [microsoft spec]
    /// for a list of language identifiers
    ///
    /// [microsoft spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn from_language_id(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "javascript" => JsFileSource::js_module().into(),
            "typescript" => JsFileSource::ts().into(),
            "javascriptreact" => JsFileSource::jsx().into(),
            "typescriptreact" => JsFileSource::tsx().into(),
            "json" => JsonFileSource::json().into(),
            "jsonc" => JsonFileSource::json().into(),
            "astro" => JsFileSource::astro().into(),
            "vue" => JsFileSource::vue().into(),
            "svelte" => JsFileSource::svelte().into(),
            // TODO: remove this when we are ready to handle CSS files
            "css" => DocumentFileSource::Unknown,
            _ => DocumentFileSource::Unknown,
        }
    }

    pub fn from_known_filename(s: &str) -> Self {
        if Self::KNOWN_FILES_AS_JSONC.binary_search(&s).is_ok() {
            JsonFileSource::jsonc().into()
        } else {
            DocumentFileSource::Unknown
        }
    }

    /// Returns the language corresponding to the file path
    pub fn from_path(path: &Path) -> Self {
        let extension = match path {
            _ if path.to_str().is_some_and(|p| p.ends_with(".d.ts")) => Some("d.ts"),
            _ if path.to_str().is_some_and(|p| p.ends_with(".d.mts")) => Some("d.mts"),
            _ if path.to_str().is_some_and(|p| p.ends_with(".d.cts")) => Some("d.cts"),
            path => path.extension().and_then(|path| path.to_str()),
        };

        extension.map_or(
            DocumentFileSource::Unknown,
            DocumentFileSource::from_extension,
        )
    }

    /// Returns the language corresponding to the file path
    /// relying on the file extension and the known files.
    pub fn from_path_and_known_filename(path: &Path) -> Self {
        let extension = match path {
            _ if path.to_str().is_some_and(|p| p.ends_with(".d.ts")) => Some("d.ts"),
            _ if path.to_str().is_some_and(|p| p.ends_with(".d.mts")) => Some("d.mts"),
            _ if path.to_str().is_some_and(|p| p.ends_with(".d.cts")) => Some("d.cts"),
            path => path.extension().and_then(|path| path.to_str()),
        };

        extension
            .map(DocumentFileSource::from_extension)
            .or(path
                .file_name()
                .and_then(OsStr::to_str)
                .map(DocumentFileSource::from_known_filename))
            .unwrap_or_default()
    }

    /// Returns the language if it's not unknown, otherwise returns `other`.
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
            DocumentFileSource::Json(_)
            | DocumentFileSource::Css(_)
            | DocumentFileSource::Unknown => None,
        }
    }

    pub fn to_json_file_source(&self) -> Option<JsonFileSource> {
        match self {
            DocumentFileSource::Json(json) => Some(*json),
            DocumentFileSource::Js(_)
            | DocumentFileSource::Css(_)
            | DocumentFileSource::Unknown => None,
        }
    }

    pub fn to_css_file_source(&self) -> Option<CssFileSource> {
        match self {
            DocumentFileSource::Css(css) => Some(*css),
            DocumentFileSource::Js(_)
            | DocumentFileSource::Json(_)
            | DocumentFileSource::Unknown => None,
        }
    }

    pub fn can_parse(path: &Path, content: &str) -> bool {
        let file_source = DocumentFileSource::from_path(path);
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
                if json.is_jsonc() {
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

// TODO: The Css variant is unused at the moment
#[allow(dead_code)]
pub(crate) enum Mime {
    Javascript,
    Json,
    Css,
    Text,
}

impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mime::Css => write!(f, "text/css"),
            Mime::Json => write!(f, "application/json"),
            Mime::Javascript => write!(f, "application/javascript"),
            Mime::Text => write!(f, "text/plain"),
        }
    }
}

impl biome_console::fmt::Display for Mime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::io::Result<()> {
        write!(f, "{self}")
    }
}

pub struct FixAllParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) rules: Option<&'a Rules>,
    pub(crate) filter: AnalysisFilter<'a>,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) settings: SettingsHandle<'a>,
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

type Parse =
    fn(&BiomePath, DocumentFileSource, &str, SettingsHandle, &mut NodeCache) -> ParseResult;

#[derive(Default)]
pub struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,
}

type DebugSyntaxTree = fn(&BiomePath, AnyParse) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(AnyParse, TextSize) -> String;
type DebugFormatterIR =
    fn(&BiomePath, &DocumentFileSource, AnyParse, SettingsHandle) -> Result<String, WorkspaceError>;

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
    pub(crate) settings: SettingsHandle<'a>,
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
    pub(crate) rules: Option<&'a Rules>,
    pub(crate) settings: SettingsHandle<'a>,
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
    SettingsHandle,
) -> Result<Printed, WorkspaceError>;
type FormatRange = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    SettingsHandle,
    TextRange,
) -> Result<Printed, WorkspaceError>;
type FormatOnType = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    SettingsHandle,
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
    /// MIME types used to identify a certain language
    fn mime(&self) -> Mime;

    /// A file that can support tabs inside its content
    fn may_use_tabs(&self) -> bool {
        true
    }

    /// Capabilities that can applied to a file
    fn capabilities(&self) -> Capabilities {
        Capabilities::default()
    }

    /// How a file should be treated. Usually an asset doesn't posses a parser.
    ///
    /// An image should me parked as asset.
    fn is_asset(&self) -> bool {
        false
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
        match DocumentFileSource::from_path_and_known_filename(biome_path).or(language_hint) {
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

#[test]
fn test_order() {
    for items in DocumentFileSource::KNOWN_FILES_AS_JSONC.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
