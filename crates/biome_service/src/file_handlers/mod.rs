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
use biome_diagnostics::{Diagnostic, Severity};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_syntax::{JsFileSource, TextRange, TextSize};
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

/// Supported languages by Biome
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Language {
    /// JavaScript
    JavaScript,
    /// JSX
    JavaScriptReact,
    /// TypeScript
    TypeScript,
    /// TSX
    TypeScriptReact,
    /// JSON
    Json,
    /// JSONC
    Jsonc,
    /// CSS
    Css,
    ///
    Astro,
    ///
    Vue,
    ///
    Svelte,
    /// Any language that is not supported
    #[default]
    Unknown,
}

impl Language {
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

    /// Returns the language corresponding to this file extension
    pub fn from_extension(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "js" | "mjs" | "cjs" => Language::JavaScript,
            "jsx" => Language::JavaScriptReact,
            "ts" | "mts" | "cts" => Language::TypeScript,
            "tsx" => Language::TypeScriptReact,
            "json" => Language::Json,
            "jsonc" => Language::Jsonc,
            "astro" => Language::Astro,
            "vue" => Language::Vue,
            "svelte" => Language::Svelte,
            "css" => Language::Css,
            _ => Language::Unknown,
        }
    }

    pub fn from_known_filename(s: &str) -> Self {
        if Self::KNOWN_FILES_AS_JSONC.binary_search(&s).is_ok() {
            Language::Jsonc
        } else {
            Language::Unknown
        }
    }

    /// Returns the language corresponding to the file path
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|path| path.to_str())
            .map(Language::from_extension)
            .unwrap_or(Language::Unknown)
    }

    /// Returns the language corresponding to the file path
    /// relying on the file extension and the known files.
    pub fn from_path_and_known_filename(path: &Path) -> Self {
        path.extension()
            .and_then(OsStr::to_str)
            .map(Language::from_extension)
            .or(path
                .file_name()
                .and_then(OsStr::to_str)
                .map(Language::from_known_filename))
            .unwrap_or_default()
    }

    /// Returns the language corresponding to this language ID
    ///
    /// See the [microsoft spec]
    /// for a list of language identifiers
    ///
    /// [microsoft spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn from_language_id(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "javascript" => Language::JavaScript,
            "typescript" => Language::TypeScript,
            "javascriptreact" => Language::JavaScriptReact,
            "typescriptreact" => Language::TypeScriptReact,
            "json" => Language::Json,
            "jsonc" => Language::Jsonc,
            "astro" => Language::Astro,
            "vue" => Language::Vue,
            "svelte" => Language::Svelte,
            // TODO: remove this when we are ready to handle CSS files
            "css" => Language::Unknown,
            _ => Language::Unknown,
        }
    }

    /// Returns the language if it's not unknown, otherwise returns `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use biome_service::workspace::Language;
    /// let x = Language::JavaScript;
    /// let y = Language::Unknown;
    /// assert_eq!(x.or(y), Language::JavaScript);
    ///
    /// let x = Language::Unknown;
    /// let y = Language::JavaScript;
    /// assert_eq!(x.or(y), Language::JavaScript);
    ///
    /// let x = Language::JavaScript;
    /// let y = Language::Json;
    /// assert_eq!(x.or(y), Language::JavaScript);
    ///
    /// let x = Language::Unknown;
    /// let y = Language::Unknown;
    /// assert_eq!(x.or(y), Language::Unknown);
    /// ```
    pub fn or(self, other: Language) -> Language {
        if self != Language::Unknown {
            self
        } else {
            other
        }
    }

    pub const fn is_javascript_like(&self) -> bool {
        matches!(
            self,
            Language::JavaScript
                | Language::TypeScript
                | Language::JavaScriptReact
                | Language::TypeScriptReact
                | Language::Astro
                | Language::Vue
        )
    }

    pub const fn is_json_like(&self) -> bool {
        matches!(self, Language::Json | Language::Jsonc)
    }

    pub const fn is_css_like(&self) -> bool {
        matches!(self, Language::Css)
    }

    pub fn as_js_file_source(&self) -> Option<JsFileSource> {
        match self {
            Language::JavaScript => Some(JsFileSource::js_module()),
            Language::JavaScriptReact => Some(JsFileSource::jsx()),
            Language::TypeScript => Some(JsFileSource::tsx()),
            Language::TypeScriptReact => Some(JsFileSource::tsx()),
            Language::Astro => Some(JsFileSource::ts()),
            Language::Vue => Some(JsFileSource::ts()),
            Language::Svelte => Some(JsFileSource::ts()),
            Language::Json | Language::Jsonc | Language::Css | Language::Unknown => None,
        }
    }

    pub fn can_parse(path: &Path, content: &str) -> bool {
        let language = Language::from_path(path);
        match language {
            Language::JavaScript
            | Language::JavaScriptReact
            | Language::TypeScript
            | Language::TypeScriptReact
            | Language::Json
            | Language::Css
            | Language::Jsonc => true,
            Language::Astro => ASTRO_FENCE.is_match(content),
            Language::Vue => VUE_FENCE.is_match(content),
            Language::Svelte => SVELTE_FENCE.is_match(content),
            Language::Unknown => false,
        }
    }
}

impl biome_console::fmt::Display for Language {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            Language::JavaScript => fmt.write_markup(markup! { "JavaScript" }),
            Language::JavaScriptReact => fmt.write_markup(markup! { "JSX" }),
            Language::TypeScript => fmt.write_markup(markup! { "TypeScript" }),
            Language::TypeScriptReact => fmt.write_markup(markup! { "TSX" }),
            Language::Json => fmt.write_markup(markup! { "JSON" }),
            Language::Jsonc => fmt.write_markup(markup! { "JSONC" }),
            Language::Css => fmt.write_markup(markup! { "CSS" }),
            Language::Astro => fmt.write_markup(markup! { "Astro" }),
            Language::Vue => fmt.write_markup(markup! { "Vue" }),
            Language::Svelte => fmt.write_markup(markup! { "Svelte" }),
            Language::Unknown => fmt.write_markup(markup! { "Unknown" }),
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
    pub(crate) language: Language,
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
    pub(crate) language: Option<Language>,
}

type Parse = fn(&BiomePath, Language, &str, SettingsHandle, &mut NodeCache) -> ParseResult;

#[derive(Default)]
pub struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,
}

type DebugSyntaxTree = fn(&BiomePath, AnyParse) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(AnyParse, TextSize) -> String;
type DebugFormatterIR = fn(&BiomePath, AnyParse, SettingsHandle) -> Result<String, WorkspaceError>;

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
    pub(crate) language: Language,
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
    pub(crate) language: Language,
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

type Format = fn(&BiomePath, AnyParse, SettingsHandle) -> Result<Printed, WorkspaceError>;
type FormatRange =
    fn(&BiomePath, AnyParse, SettingsHandle, TextRange) -> Result<Printed, WorkspaceError>;
type FormatOnType =
    fn(&BiomePath, AnyParse, SettingsHandle, TextSize) -> Result<Printed, WorkspaceError>;

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
    /// The language of the file. It can be a super language.
    /// For example, a ".js" file can have [Language::Ts]
    fn language(&self) -> Language;

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
        language_hint: Language,
    ) -> Capabilities {
        match Language::from_path_and_known_filename(biome_path).or(language_hint) {
            Language::JavaScript
            | Language::JavaScriptReact
            | Language::TypeScript
            | Language::TypeScriptReact => self.js.capabilities(),
            Language::Json | Language::Jsonc => self.json.capabilities(),
            Language::Css => {
                // TODO: change this when we are ready to handle CSS files
                if can_format_css_yet() {
                    self.css.capabilities()
                } else {
                    self.unknown.capabilities()
                }
            }
            Language::Astro => self.astro.capabilities(),
            Language::Vue => self.vue.capabilities(),
            Language::Svelte => self.svelte.capabilities(),
            Language::Unknown => self.unknown.capabilities(),
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
        .map(|category| {
            rules
                .and_then(|rules| rules.get_severity_from_code(category))
                .unwrap_or(Severity::Warning)
        })
        .unwrap_or_else(|| diagnostic.severity());

    severity >= Severity::Error
}

#[test]
fn test_order() {
    for items in Language::KNOWN_FILES_AS_JSONC.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
