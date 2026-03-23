use crate::WorkspaceError;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities, EnabledForPath,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, javascript,
};
use crate::settings::SettingsWithEditor;
use crate::workspace::{DocumentFileSource, FixFileResult, PullActionsResult};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{JsParserOptions, parse_js_with_cache};
use biome_js_syntax::{JsFileSource, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use regex::{Matches, Regex, RegexBuilder};
use std::ops::Range;
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
        Self::content_range(text).map_or("", |range| &text[range])
    }

    /// Returns the start byte offset of the Astro fence
    pub fn start(input: &str) -> Option<u32> {
        Self::content_range(input).map(|range| range.start as u32)
    }

    fn matches(input: &str) -> Matches<'_, '_> {
        ASTRO_FENCE.find_iter(input)
    }

    /// It takes the original content of an Astro file, and new output of an Astro file. The output is only the content contained inside the
    /// Astro fences. The function replaces `output` inside those fences.
    pub fn output(input: &str, output: &str) -> String {
        if let Some(range) = Self::content_range(input) {
            format!("{}{}{}", &input[..range.start], output, &input[range.end..])
        } else {
            input.to_string()
        }
    }

    /// Returns the byte range of the actual frontmatter code inside the `---` fences.
    ///
    /// Astro frontmatter commonly includes a newline immediately after the opening
    /// fence and before the closing fence. Those wrapper-adjacent bytes are part of
    /// the host document layout, but they should not be treated as part of the
    /// embedded JavaScript/TypeScript snippet itself.
    ///
    /// Keeping them in the embedded slice makes snippet-based assists like
    /// `organizeImports` observe a different input than standalone JS/TS, which can
    /// lead to incorrect blank-line edits at the snippet boundary. By trimming only
    /// the fence-adjacent whitespace, we keep parser offsets and fix application
    /// aligned to the meaningful frontmatter content while still preserving the host
    /// document's surrounding layout.
    fn content_range(input: &str) -> Option<Range<usize>> {
        let mut matches = Self::matches(input);
        let (start, end) = (matches.next()?, matches.next()?);
        let content = &input[start.end()..end.start()];
        let leading_whitespace = content
            .bytes()
            .take_while(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
            .count();
        let trailing_whitespace = content
            .bytes()
            .rev()
            .take_while(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
            .count();
        let content_start = start.end() + leading_whitespace;
        let content_end = if leading_whitespace == content.len() {
            content_start
        } else {
            end.start() - trailing_whitespace
        };

        Some(content_start..content_end)
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

            parser: ParserCapabilities {
                parse: Some(parse),
                parse_embedded_nodes: None,
            },
            debug: DebugCapabilities {
                debug_syntax_tree: None,
                debug_control_flow: None,
                debug_formatter_ir: None,
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                rename: None,
                fix_all: Some(fix_all),
                update_snippets: None,
                pull_diagnostics_and_actions: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
                format_embedded: None,
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
    _settings: &SettingsWithEditor,
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
    settings: &SettingsWithEditor,
) -> Result<Printed, WorkspaceError> {
    javascript::format(biome_path, document_file_source, parse, settings)
}
pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    javascript::format_range(biome_path, document_file_source, parse, settings, range)
}

pub(crate) fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
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
