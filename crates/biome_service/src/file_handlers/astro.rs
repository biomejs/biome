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
use regex::{Regex, RegexBuilder};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AstroFileHandler;

lazy_static! {
    pub static ref ASTRO_FENCE: Regex = RegexBuilder::new(r#"^---\s*$"#)
        .multi_line(true)
        .build()
        .unwrap();
}

impl AstroFileHandler {
    /// It extracts the JavaScript code contained in the frontmatter of an Astro file
    ///
    /// If the frontmatter doesn't exist, an empty string is returned.
    pub fn astro_input(text: &str) -> &str {
        let mut matches = ASTRO_FENCE.find_iter(text);
        match (matches.next(), matches.next()) {
            (Some(start), Some(end)) => &text[start.end()..end.start()],
            _ => "",
        }
    }

    pub fn astro_output(input: &str, output: &str) -> String {
        let mut matches = ASTRO_FENCE.find_iter(input);
        if let (Some(start), Some(end)) = (matches.next(), matches.next()) {
            format!(
                "{}{}{}",
                &input[..start.end() + 1],
                output.trim_start(),
                &input[end.start()..]
            )
        } else {
            input.to_string()
        }
    }
}

impl ExtensionHandler for AstroFileHandler {
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
    let frontmatter = AstroFileHandler::astro_input(text);
    let parse = parse_js_with_cache(
        frontmatter,
        JsFileSource::ts(),
        JsParserOptions::default(),
        cache,
    );
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
