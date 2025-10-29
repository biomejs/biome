use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities, EnabledForPath,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, SearchCapabilities, javascript,
};
use crate::settings::Settings;
use crate::workspace::{DocumentFileSource, FixFileResult, PullActionsResult};
use crate::WorkspaceError;
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::{JsFileSource, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use regex::Regex;
use std::sync::LazyLock;

/// Regex to match Glimmer <template> tags
/// Simple pattern: <template> never has attributes and never nests
pub static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>")
        .expect("Invalid Glimmer template regex")
});

#[derive(Debug, Default, PartialEq, Eq)]
pub struct GlimmerFileHandler;

impl GlimmerFileHandler {
    /// Extract the JavaScript/TypeScript code with <template> blocks removed
    ///
    /// This allows us to parse the JS/TS part of GJS/GTS files without
    /// the template syntax interfering. Templates are replaced with whitespace
    /// to maintain correct offsets.
    pub fn extract_js_content(text: &str) -> String {
        GLIMMER_TEMPLATE.replace_all(text, |caps: &regex::Captures| {
            // Replace template content with equivalent whitespace to maintain offsets
            " ".repeat(caps.get(0).unwrap().as_str().len())
        }).to_string()
    }

    /// Check if the file contains any <template> blocks
    pub fn has_templates(text: &str) -> bool {
        GLIMMER_TEMPLATE.is_match(text)
    }
}

impl ExtensionHandler for GlimmerFileHandler {
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
                // TODO: Implement parse_embedded_nodes to extract <template> blocks
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
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
                format_embedded: None,
            },
            search: SearchCapabilities { search: None },
        }
    }
}

/// Parse GJS/GTS file as JavaScript/TypeScript
/// 
/// Templates are stripped out before parsing to avoid syntax errors.
/// In the future, templates will be parsed separately with the HTML parser.
fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let js_file_source = file_source
        .to_js_file_source()
        .unwrap_or_else(|| {
            // Determine if this is GJS or GTS based on extension
            if let Some(ext) = biome_path.extension() {
                if ext == "gts" {
                    return JsFileSource::gts();
                }
            }
            JsFileSource::gjs()
        });

    // Extract JS content with templates replaced by whitespace
    let js_content = GlimmerFileHandler::extract_js_content(text);

    let parse = parse_js_with_cache(
        &js_content,
        js_file_source,
        JsParserOptions::default(),
        cache,
    );

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    // For now, delegate to JavaScript formatter
    // TODO: Extract and format <template> blocks separately
    javascript::format(biome_path, document_file_source, parse, settings)
}

fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    javascript::format_range(biome_path, document_file_source, parse, settings, range)
}

fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    javascript::format_on_type(biome_path, document_file_source, parse, settings, offset)
}

fn lint(params: LintParams) -> LintResults {
    // For now, delegate to JavaScript linter
    // TODO: Also lint <template> blocks with Glimmer rules
    javascript::lint(params)
}

fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    javascript::code_actions(params)
}

fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    javascript::fix_all(params)
}
