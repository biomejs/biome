use crate::file_handlers::{
    javascript, AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities,
    ExtensionHandler, FixAllParams, FormatterCapabilities, Language, LintParams, LintResults, Mime,
    ParseResult, ParserCapabilities,
};
use crate::settings::SettingsHandle;
use crate::workspace::{FixFileResult, OrganizeImportsResult, PullActionsResult};
use crate::WorkspaceError;
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::{JsFileSource, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use tracing::debug;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct VueFileHandler;

lazy_static! {
    // https://regex101.com/r/E4n4hh/3
    pub static ref VUE_FENCE: Regex = Regex::new(
        r#"(?ixms)(?:<script[^>]?)
            (?:
            (?:(lang)\s*=\s*['"](?P<lang>[^'"]*)['"])
            |
            (?:(\w+)\s*(?:=\s*['"]([^'"]*)['"])?)
            )*
        [^>]*>\n(?P<script>(?U:.*))</script>"#
    )
    .unwrap();
}

impl VueFileHandler {
    /// It extracts the JavaScript/TypeScript code contained in the script block of a Vue file
    ///
    /// If there's no script block, an empty string is returned.
    pub fn input(text: &str) -> &str {
        match Self::matches_script(text) {
            Some(script) => &text[script.start()..script.end()],
            _ => "",
        }
    }

    /// It takes the original content of a Vue file, and new output of an Vue file. The output is only the content contained inside the
    /// Vue `<script>` tag. The function replaces `output` inside that `<script>`.
    pub fn output(input: &str, output: &str) -> String {
        if let Some(script) = Self::matches_script(input) {
            format!(
                "{}{}{}",
                &input[..script.start()],
                output,
                &input[script.end()..]
            )
        } else {
            input.to_string()
        }
    }

    /// Returns the start byte offset of the Vue `<script>` tag
    pub fn start(input: &str) -> Option<u32> {
        Self::matches_script(input).map(|m| m.start() as u32 - 1)
    }

    fn matches_script(input: &str) -> Option<Match> {
        VUE_FENCE
            .captures(input)
            .and_then(|captures| captures.name("script"))
    }

    pub fn file_source(text: &str) -> JsFileSource {
        let matches = VUE_FENCE.captures(text);
        matches
            .and_then(|captures| captures.name("lang"))
            .map(|lang| match lang.as_str() {
                "ts" => JsFileSource::ts(),
                _ => JsFileSource::js_module(),
            })
            .unwrap_or(JsFileSource::js_module())
    }
}

impl ExtensionHandler for VueFileHandler {
    fn language(&self) -> Language {
        Language::TypeScript
    }

    fn mime(&self) -> Mime {
        Mime::Javascript
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: None,
                debug_control_flow: None,
                debug_formatter_ir: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                rename: None,
                fix_all: Some(fix_all),
                organize_imports: Some(organize_imports),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
        }
    }
}

fn parse(
    _rome_path: &BiomePath,
    _language_hint: Language,
    text: &str,
    _settings: SettingsHandle,
    cache: &mut NodeCache,
) -> ParseResult {
    let script = VueFileHandler::input(text);
    let language = VueFileHandler::file_source(text);

    debug!("Parsing file with language {:?}", language);

    let parse = parse_js_with_cache(script, language, JsParserOptions::default(), cache);
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();

    ParseResult {
        any_parse: AnyParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        ),
        language: Some(if language.is_typescript() {
            Language::TypeScript
        } else {
            Language::JavaScript
        }),
    }
}

#[tracing::instrument(level = "trace", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<Printed, WorkspaceError> {
    javascript::format(biome_path, parse, settings)
}

pub(crate) fn format_range(
    biome_path: &BiomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    javascript::format_range(biome_path, parse, settings, range)
}

pub(crate) fn format_on_type(
    biome_path: &BiomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    javascript::format_on_type(biome_path, parse, settings, offset)
}

pub(crate) fn lint(params: LintParams) -> LintResults {
    javascript::lint(params)
}

pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    javascript::code_actions(params)
}

fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    javascript::fix_all(params)
}

fn organize_imports(parse: AnyParse) -> Result<OrganizeImportsResult, WorkspaceError> {
    javascript::organize_imports(parse)
}
