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
use regex::{Matches, Regex, RegexBuilder};

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
    pub fn input(text: &str) -> &str {
        let mut matches = Self::matches(text);
        match (matches.next(), matches.next()) {
            (Some(start), Some(end)) => &text[start.end()..end.start()],
            _ => "",
        }
    }

    /// Returns the start byte offset of the Astro fence
    pub fn start(input: &str) -> Option<u32> {
        ASTRO_FENCE.find_iter(input).next().map(|m| m.end() as u32)
    }

    fn matches(input: &str) -> Matches {
        ASTRO_FENCE.find_iter(input)
    }

    /// It takes the original content of an Astro file, and new output of an Astro file. The output is only the content contained inside the
    /// Astro fences. The function replaces `output` inside those fences.
    pub fn output(input: &str, output: &str) -> String {
        let mut matches = Self::matches(input);
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
    let frontmatter = AstroFileHandler::input(text);
    let parse = parse_js_with_cache(
        frontmatter,
        JsFileSource::ts(),
        JsParserOptions::default(),
        cache,
    );
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();

    ParseResult {
        any_parse: AnyParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        ),
        language: Some(Language::TypeScript),
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
