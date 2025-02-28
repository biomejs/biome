use crate::file_handlers::{
    javascript, AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities,
    EnabledForPath, ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults,
    ParseResult, ParserCapabilities,
};
use crate::settings::WorkspaceSettingsHandle;
use crate::workspace::{DocumentFileSource, FixFileResult, PullActionsResult};
use crate::WorkspaceError;
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::{JsFileSource, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use regex::{Matches, Regex, RegexBuilder};
use std::sync::LazyLock;

use super::SearchCapabilities;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AstroFileHandler;

pub static ASTRO_FENCE: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r#"^---\s*$"#)
        .multi_line(true)
        .build()
        .unwrap()
});

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
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(javascript::formatter_enabled),
                search: Some(javascript::search_enabled),
                assist: Some(javascript::assist_enabled),
                linter: Some(javascript::linter_enabled),
            },

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
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
            // TODO: We should be able to search JS portions already
            search: SearchCapabilities { search: None },
        }
    }
}

fn parse(
    _rome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: WorkspaceSettingsHandle,
    cache: &mut NodeCache,
) -> ParseResult {
    let frontmatter = AstroFileHandler::input(text);
    let parse = parse_js_with_cache(
        frontmatter,
        file_source
            .to_js_file_source()
            .unwrap_or(JsFileSource::ts()),
        JsParserOptions::default(),
        cache,
    );

    ParseResult {
        any_parse: parse.into(),
        language: Some(JsFileSource::astro().into()),
    }
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    javascript::format(biome_path, document_file_source, parse, settings)
}
pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    javascript::format_range(biome_path, document_file_source, parse, settings, range)
}

pub(crate) fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    javascript::format_on_type(biome_path, document_file_source, parse, settings, offset)
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
