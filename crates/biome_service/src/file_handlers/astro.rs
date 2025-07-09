use super::{
    AnalyzerCapabilities, Capabilities, DebugCapabilities, DocumentFileSource, EnabledForPath,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, SearchCapabilities, html,
};
use crate::settings::Settings;
use crate::workspace::{GetSyntaxTreeResult, FixFileResult};
use crate::WorkspaceError;
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_html_parser::parse_html_with_cache;
use biome_html_syntax::{HtmlRoot, HtmlSyntaxNode};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use regex::{Matches, Regex, RegexBuilder};
use std::sync::LazyLock;

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
                formatter: Some(html::formatter_enabled),
                search: Some(html::search_enabled),
                assist: Some(html::assist_enabled),
                linter: Some(html::linter_enabled),
            },

            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: Some(debug_formatter_ir),
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: None,
                rename: None,
                fix_all: Some(fix_all),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: None,
                format_on_type: None,
            },
            search: SearchCapabilities { search: None },
        }
    }
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let html_file_source = file_source.to_html_file_source().unwrap_or_default();
    let parse = parse_html_with_cache(text, html_file_source, cache);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

fn debug_syntax_tree(_biome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: HtmlSyntaxNode = parse.syntax();
    let tree: HtmlRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<String, WorkspaceError> {
    html::debug_formatter_ir(path, document_file_source, parse, settings)
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    html::format(biome_path, document_file_source, parse, settings)
}

#[tracing::instrument(level = "debug", skip(params))]
fn lint(params: LintParams) -> LintResults {
    html::lint(params)
}

#[tracing::instrument(level = "debug", skip(params))]
fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    html::fix_all(params)
}
