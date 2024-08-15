use crate::file_handlers::{
    javascript, AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities,
};
use crate::settings::{Settings, WorkspaceSettingsHandle};
use crate::workspace::{
    DocumentFileSource, FixFileResult, OrganizeImportsResult, PullActionsResult,
};
use crate::WorkspaceError;
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::{EmbeddingKind, JsFileSource, Language, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use regex::{Match, Regex};
use std::sync::LazyLock;
use tracing::debug;

use super::{parse_lang_from_script_opening_tag, SearchCapabilities};

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

    fn matches_script(input: &str) -> Option<Match> {
        VUE_FENCE
            .captures(input)
            .and_then(|captures| captures.name("script"))
    }

    pub fn file_source(text: &str) -> JsFileSource {
        VUE_FENCE
            .captures(text)
            .and_then(|captures| {
                match parse_lang_from_script_opening_tag(captures.name("opening")?.as_str()) {
                    Language::JavaScript => {
                        Some(JsFileSource::js_module().with_embedding_kind(EmbeddingKind::Vue))
                    }
                    Language::TypeScript { .. } => {
                        Some(JsFileSource::ts().with_embedding_kind(EmbeddingKind::Vue))
                    }
                }
            })
            .map_or(JsFileSource::js_module(), |fs| fs)
    }
}

impl ExtensionHandler for VueFileHandler {
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
            // TODO: We should be able to search JS portions already
            search: SearchCapabilities { search: None },
        }
    }
}

fn parse(
    _rome_path: &BiomePath,
    _file_source: DocumentFileSource,
    text: &str,
    _settings: Option<&Settings>,
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

#[tracing::instrument(level = "trace", skip(parse, settings))]
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

fn organize_imports(parse: AnyParse) -> Result<OrganizeImportsResult, WorkspaceError> {
    javascript::organize_imports(parse)
}
