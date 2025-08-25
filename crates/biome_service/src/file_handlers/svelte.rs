use crate::WorkspaceError;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities, EnabledForPath,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, javascript, html,
};
use crate::settings::Settings;
use crate::workspace::{DocumentFileSource, FixFileResult, PullActionsResult, EmbeddedJsContent};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_html_parser::parse_html_with_cache;
use biome_html_syntax::{HtmlFileSource, HtmlRoot};
use biome_js_parser::{JsParserOptions, parse_js_with_cache};
use biome_js_syntax::{EmbeddingKind, JsFileSource, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache};
use regex::{Match, Regex};
use std::sync::LazyLock;
use tracing::debug;

use super::{SearchCapabilities, parse_lang_from_script_opening_tag};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SvelteFileHandler;

// Kept for backward compatibility with existing code
pub static SVELTE_FENCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?ixs)(?<opening><script(?:\s.*?)?>)\r?\n?(?<script>(?U:.*?))</script>"#).unwrap()
});

impl SvelteFileHandler {
    /// Extracts the JavaScript/TypeScript code contained in the script block of a Svelte file
    /// using HTML parsing instead of regex for better handling of HTML structure.
    ///
    /// If there's no script block, an empty string is returned.
    pub fn input(text: &str) -> &str {
        // For backwards compatibility, still use the regex approach
        match Self::matches_script(text) {
            Some(script) => &text[script.start()..script.end()],
            _ => "",
        }
    }

    /// Extracts all embedded JavaScript content from a Svelte file using HTML parsing.
    /// This provides better handling of HTML structure and script indentation.
    pub fn extract_scripts(text: &str, cache: &mut NodeCache) -> Vec<EmbeddedJsContent> {
        // Parse the Svelte file as HTML
        let html_file_source = HtmlFileSource::default();
        let parse = parse_html_with_cache(text, html_file_source, cache);
        let html_root = HtmlRoot::cast(parse.syntax()).expect("Failed to cast to HtmlRoot");
        
        // Extract embedded scripts using the HTML handler
        html::extract_embedded_scripts(&html_root, cache)
    }

    /// It takes the original content of a Svelte file, and new output of an Svelte file. The output is only the content contained inside the
    /// Svelte `<script>` tag. The function replaces `output` inside that `<script>`.
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

    /// Returns the start byte offset of the Svelte `<script>` tag
    pub fn start(input: &str) -> Option<u32> {
        Self::matches_script(input).map(|m| m.start() as u32)
    }

    fn matches_script(input: &str) -> Option<Match<'_>> {
        SVELTE_FENCE
            .captures(input)
            .and_then(|captures| captures.name("script"))
    }

    pub fn file_source(text: &str) -> JsFileSource {
        SVELTE_FENCE
            .captures(text)
            .and_then(|captures| {
                let (language, variant) =
                    parse_lang_from_script_opening_tag(captures.name("opening")?.as_str());
                Some(
                    JsFileSource::from(language)
                        .with_variant(variant)
                        .with_embedding_kind(EmbeddingKind::Svelte),
                )
            })
            .map_or(JsFileSource::js_module(), |fs| fs)
    }
}

impl ExtensionHandler for SvelteFileHandler {
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
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
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
    _settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    // Use HTML parsing to extract script content with proper context
    let scripts = SvelteFileHandler::extract_scripts(text, cache);
    
    // If we found scripts, use the first one; otherwise fall back to regex approach
    if let Some(first_script) = scripts.first() {
        debug!("Parsing Svelte file with HTML-extracted script");
        
        ParseResult {
            any_parse: first_script.parse.clone(),
            language: Some(file_source),
        }
    } else {
        // Fallback to the old regex approach
        let script = SvelteFileHandler::input(text);
        let js_file_source = SvelteFileHandler::file_source(text);
        
        debug!("Parsing file with language {:?} (fallback)", js_file_source);
        
        let parse = parse_js_with_cache(script, js_file_source, JsParserOptions::default(), cache);
        
        ParseResult {
            any_parse: parse.into(),
            language: Some(js_file_source.into()),
        }
    }
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    // Use JavaScript formatter but preserve HTML context
    javascript::format(biome_path, document_file_source, parse, settings)
}
pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    javascript::format_range(biome_path, document_file_source, parse, settings, range)
}

pub(crate) fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
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
