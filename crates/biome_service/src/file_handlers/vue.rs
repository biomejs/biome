use crate::file_handlers::{
    javascript, AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities,
    ExtensionHandler, FixAllParams, FormatterCapabilities, Language, LintParams, LintResults, Mime,
    ParserCapabilities,
};
use crate::settings::SettingsHandle;
use crate::workspace::{FixFileResult, PullActionsResult};
use crate::WorkspaceError;
use biome_formatter::Printed;
use biome_fs::RomePath;
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::JsFileSource;
use biome_parser::AnyParse;
use biome_rowan::{FileSource, NodeCache};
use lazy_static::lazy_static;
use regex::Regex;
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
    pub fn vue_input(text: &str) -> &str {
        let script = VUE_FENCE
            .captures(text)
            .and_then(|captures| captures.name("script"));
        match script {
            Some(script) => &text[script.start()..script.end()],
            _ => "",
        }
    }

    pub fn vue_output(input: &str, output: &str) -> String {
        if let Some(script) = VUE_FENCE
            .captures(input)
            .and_then(|captures| captures.name("script"))
        {
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

    pub fn vue_file_source(text: &str) -> JsFileSource {
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
                organize_imports: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: None,
                format_on_type: None,
            },
        }
    }
}

fn parse(
    _rome_path: &RomePath,
    _language_hint: Language,
    text: &str,
    _settings: SettingsHandle,
    cache: &mut NodeCache,
) -> AnyParse {
    let script = VueFileHandler::vue_input(text);
    let language = VueFileHandler::vue_file_source(text);

    debug!("Parsing file with language {:?}", language);

    let parse = parse_js_with_cache(script, language, JsParserOptions::default(), cache);
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();

    AnyParse::new(
        // SAFETY: the parser should always return a root node
        root.as_send().unwrap(),
        diagnostics,
        JsFileSource::ts().as_any_file_source(),
    )
}

#[tracing::instrument(level = "trace", skip(parse, settings))]
fn format(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<Printed, WorkspaceError> {
    javascript::format(rome_path, parse, settings)
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
