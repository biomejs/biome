use super::{SearchCapabilities, parse_lang_from_script_opening_tag};
use crate::WorkspaceError;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities, EnabledForPath,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, javascript,
};
use crate::settings::Settings;
use crate::workspace::{DocumentFileSource, FixFileResult, PullActionsResult};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_html_syntax::HtmlLanguage;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse_js_with_cache};
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use regex::{Match, Regex};
use std::sync::LazyLock;
use tracing::{debug, error};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct VueFileHandler;

// https://regex101.com/r/E4n4hh/6
pub static VUE_FENCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?ixs)(?<opening><script(?:\s.*?)?>)\r?\n(?<script>(?U:.*))</script>"#).unwrap()
});

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
        Self::matches_script(input).map(|m| m.start() as u32)
    }

    fn matches_script(input: &str) -> Option<Match<'_>> {
        VUE_FENCE
            .captures(input)
            .and_then(|captures| captures.name("script"))
    }

    pub fn file_source(text: &str) -> JsFileSource {
        VUE_FENCE
            .captures(text)
            .and_then(|captures| {
                let (language, variant) =
                    parse_lang_from_script_opening_tag(captures.name("opening")?.as_str());
                Some(
                    JsFileSource::from(language)
                        .with_variant(variant)
                        .with_embedding_kind(EmbeddingKind::Vue),
                )
            })
            .map_or(JsFileSource::js_module(), |fs| fs)
    }
}

impl ExtensionHandler for VueFileHandler {
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
    _file_source: DocumentFileSource,
    text: &str,
    _settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let script = VueFileHandler::input(text);
    let file_source = VueFileHandler::file_source(text);

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
    let html_options = settings.format_options::<HtmlLanguage>(biome_path, document_file_source);
    let indent_amount = if *html_options.indent_script_and_style() {
        1
    } else {
        0
    };
    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;
    match formatted.print_with_indent(indent_amount) {
        Ok(printed) => Ok(printed),
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
