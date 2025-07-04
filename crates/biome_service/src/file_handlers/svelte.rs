use crate::WorkspaceError;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities, EnabledForPath,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, javascript,
};
use crate::settings::{FormatSettings, LanguageListSettings, LanguageSettings, ServiceLanguage, Settings};
use crate::workspace::{DocumentFileSource, FixFileResult, PullActionsResult};
use biome_formatter::{Printed, IndentStyle, IndentWidth, LineEnding, LineWidth, BracketSpacing, BracketSameLine, AttributePosition, Expand};
use biome_fs::BiomePath;
use biome_js_formatter::context::{ArrowParentheses, JsFormatOptions, QuoteProperties, Semicolons, TrailingCommas};
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse_js_with_cache};
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use regex::{Match, Regex};
use std::sync::LazyLock;
use tracing::debug;
use biome_formatter::QuoteStyle;
use crate::configuration::to_analyzer_rules;
use crate::settings::OverrideSettings;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use super::{SearchCapabilities, parse_lang_from_script_opening_tag};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SvelteFormatterSettings {
    pub quote_style: Option<QuoteStyle>,
    pub jsx_quote_style: Option<QuoteStyle>,
    pub quote_properties: Option<QuoteProperties>,
    pub trailing_commas: Option<TrailingCommas>,
    pub semicolons: Option<Semicolons>,
    pub arrow_parentheses: Option<ArrowParentheses>,
    pub bracket_spacing: Option<BracketSpacing>,
    pub bracket_same_line: Option<BracketSameLine>,
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub attribute_position: Option<AttributePosition>,
    pub expand: Option<Expand>,
}

impl ServiceLanguage for JsLanguage {
    type FormatterSettings = SvelteFormatterSettings;
    type LinterSettings = javascript::JsLinterSettings;
    type ParserSettings = javascript::JsParserSettings;
    type AssistSettings = javascript::JsAssistSettings;
    type EnvironmentSettings = javascript::JsEnvironmentSettings;

    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self> {
        &languages.svelte
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &SvelteFormatterSettings,
        path: &BiomePath,
        document_file_source: &DocumentFileSource,
    ) -> JsFormatOptions {
        let options = JsFormatOptions::new(
            document_file_source
                .to_js_file_source()
                .or(JsFileSource::try_from(path.as_path()).ok())
                .unwrap_or_default(),
        )
        .with_indent_style(
            language
                .indent_style
                .or(global.indent_style)
                .unwrap_or_default(),
        )
        .with_indent_width(
            language
                .indent_width
                .or(global.indent_width)
                .unwrap_or_default(),
        )
        .with_line_width(
            language
                .line_width
                .or(global.line_width)
                .unwrap_or_default(),
        )
        .with_line_ending(
            language
                .line_ending
                .or(global.line_ending)
                .unwrap_or_default(),
        )
        .with_quote_style(language.quote_style.unwrap_or_default())
        .with_jsx_quote_style(language.jsx_quote_style.unwrap_or_default())
        .with_quote_properties(language.quote_properties.unwrap_or_default())
        .with_trailing_commas(language.trailing_commas.unwrap_or_default())
        .with_semicolons(language.semicolons.unwrap_or(Semicolons::AsNeeded)) // Default to AsNeeded for Svelte files
        .with_arrow_parentheses(language.arrow_parentheses.unwrap_or_default())
        .with_bracket_spacing(
            language
                .bracket_spacing
                .or(global.bracket_spacing)
                .unwrap_or_default(),
        )
        .with_bracket_same_line(
            language
                .bracket_same_line
                .or(global.bracket_same_line)
                .unwrap_or_default(),
        )
        .with_attribute_position(
            language
                .attribute_position
                .or(global.attribute_position)
                .unwrap_or_default(),
        )
        .with_expand(language.expand.or(global.expand).unwrap_or_default());

        overrides.override_js_format_options(path, options)
    }

    fn resolve_analyzer_options(
        global: &Settings,
        _language: &Self::LinterSettings,
        environment: Option<&Self::EnvironmentSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> biome_analyze::AnalyzerOptions {
        javascript::JsLanguage::resolve_analyzer_options(global, _language, environment, path, _file_source, suppression_reason)
    }

    fn formatter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        javascript::formatter_enabled(path, settings)
    }

    fn assist_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        javascript::assist_enabled(path, settings)
    }

    fn linter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        javascript::linter_enabled(path, settings)
    }

    fn resolve_environment(global: &Settings) -> Option<&Self::EnvironmentSettings> {
        javascript::JsLanguage::resolve_environment(global)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SvelteFileHandler;

// https://regex101.com/r/E4n4hh/6
pub static SVELTE_FENCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?ixs)(?<opening><script(?:\s.*?)?>)\r?\n?(?<script>(?U:.*))</script>"#).unwrap()
});

impl SvelteFileHandler {
    /// It extracts the JavaScript/TypeScript code contained in the script block of a Svelte file
    ///
    /// If there's no script block, an empty string is returned.
    pub fn input(text: &str) -> &str {
        match Self::matches_script(text) {
            Some(script) => &text[script.start()..script.end()],
            _ => "",
        }
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

    fn matches_script(input: &str) -> Option<Match> {
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
    _file_source: DocumentFileSource,
    text: &str,
    _settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let script = SvelteFileHandler::input(text);
    let file_source = SvelteFileHandler::file_source(text);

    debug!("Parsing file with language {:?}", file_source);

    let parse = parse_js_with_cache(script, file_source, JsParserOptions::default(), cache);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source.into()),
    }
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
    debug!("{:?}", &options);
    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;
    
    // Get the original text to handle the script tag extraction/insertion
    let original_text = document_file_source.text();
    
    // Format the content and get the printed result
    match formatted.print() {
        Ok(printed) => {
            // For Svelte files, we need to handle the script tag content
            if document_file_source.to_js_file_source().embedding_kind() == Some(EmbeddingKind::Svelte) {
                // Get the formatted content
                let formatted_content = printed.into_code();
                // Insert the formatted content back into the script tag
                let result = SvelteFileHandler::output(original_text, &formatted_content);
                Ok(Printed::from(result))
            } else {
                Ok(printed)
            }
        },
        Err(error) => {
            error!("The file {} couldn't be formatted", biome_path.as_str());
            Err(WorkspaceError::FormatError(error.into()))
        }
    }
}

pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    // For Svelte files, we need to adjust the range to be relative to the script content
    if document_file_source.to_js_file_source().embedding_kind() == Some(EmbeddingKind::Svelte) {
        let original_text = document_file_source.text();
        if let Some(script_start) = SvelteFileHandler::start(original_text) {
            // Adjust the range to be relative to the script content
            let adjusted_range = TextRange::new(
                range.start().checked_sub(TextSize::from(script_start)).unwrap_or_default(),
                range.end().checked_sub(TextSize::from(script_start)).unwrap_or_default(),
            );
            
            let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
            let tree = parse.syntax();
            let formatted = format_node(options, &tree)?;
            
            match formatted.print() {
                Ok(printed) => {
                    let formatted_content = printed.into_code();
                    let result = SvelteFileHandler::output(original_text, &formatted_content);
                    Ok(Printed::from(result))
                },
                Err(error) => {
                    error!("The file {} couldn't be formatted", biome_path.as_str());
                    Err(WorkspaceError::FormatError(error.into()))
                }
            }
        } else {
            javascript::format_range(biome_path, document_file_source, parse, settings, range)
        }
    } else {
        javascript::format_range(biome_path, document_file_source, parse, settings, range)
    }
}

pub(crate) fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    // For Svelte files, we need to adjust the offset to be relative to the script content
    if document_file_source.to_js_file_source().embedding_kind() == Some(EmbeddingKind::Svelte) {
        let original_text = document_file_source.text();
        if let Some(script_start) = SvelteFileHandler::start(original_text) {
            // Adjust the offset to be relative to the script content
            let adjusted_offset = offset.checked_sub(TextSize::from(script_start)).unwrap_or_default();
            
            let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
            let tree = parse.syntax();
            let formatted = format_node(options, &tree)?;
            
            match formatted.print() {
                Ok(printed) => {
                    let formatted_content = printed.into_code();
                    let result = SvelteFileHandler::output(original_text, &formatted_content);
                    Ok(Printed::from(result))
                },
                Err(error) => {
                    error!("The file {} couldn't be formatted", biome_path.as_str());
                    Err(WorkspaceError::FormatError(error.into()))
                }
            }
        } else {
            javascript::format_on_type(biome_path, document_file_source, parse, settings, offset)
        }
    } else {
        javascript::format_on_type(biome_path, document_file_source, parse, settings, offset)
    }
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
