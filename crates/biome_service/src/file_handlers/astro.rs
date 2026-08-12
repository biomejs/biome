use super::SearchCapabilities;
use crate::WorkspaceError;
use crate::db::WorkspaceDb;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities, EditorCapabilities,
    EnabledForPath, ExtensionHandler, FixAllParams, FixedFileResult, FormatterCapabilities,
    LintParams, LintResults, ParseResult, ParserCapabilities, javascript,
};
use crate::settings::SettingsWithEditor;
use crate::workspace::PullActionsResult;
use biome_db::{Db, FileSource};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{JsParserOptions, parse as parse_js, parse_js_with_cache};
use biome_js_syntax::{TextRange, TextSize};
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_parser::{AnyParse, AnyParsedSource};
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

    fn matches(input: &str) -> Matches<'_, '_> {
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

            parser: ParserCapabilities {
                parse: Some(parse),
                parse_detached: Some(parse_detached),
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
            editors: EditorCapabilities {
                resolve_binding: None,
                resolve_definition: None,
            },
        }
    }
}

#[salsa::interned]
struct ParseAstroInput {
    file: FileSource,
    document_source: JsFileSource,
}

#[salsa::tracked(returns(clone), no_eq)]
fn parse_astro_file<'db>(db: &'db dyn Db, input: ParseAstroInput<'db>) -> AnyParse {
    parse_js(
        AstroFileHandler::input(input.file(db).content(db)),
        input.document_source(db),
        JsParserOptions::default(),
    )
    .into()
}

fn parse(
    biome_path: &BiomePath,
    _settings: &SettingsWithEditor,
    db: WorkspaceDb,
) -> Result<ParseResult, WorkspaceError> {
    let (file, file_source) = db
        .file_and_source_from_path(biome_path.as_path())
        .ok_or_else(|| WorkspaceError::not_found(biome_path.as_path().to_string()))?;
    let file_db: &dyn Db = &db;
    let any_parse = parse_astro_file(
        file_db,
        ParseAstroInput::new(
            file_db,
            file,
            file_source
                .to_js_file_source()
                .unwrap_or(JsFileSource::ts()),
        ),
    );

    Ok(ParseResult {
        any_parse,
        language: Some(JsFileSource::astro().into()),
    })
}

fn parse_detached(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    code: &str,
    _settings: &SettingsWithEditor,
    node_cache: &mut NodeCache,
) -> ParseResult {
    let document_source = file_source
        .to_js_file_source()
        .unwrap_or(JsFileSource::ts());
    let any_parse = parse_js_with_cache(
        AstroFileHandler::input(code),
        document_source,
        JsParserOptions::default(),
        node_cache,
    )
    .into();

    ParseResult {
        any_parse,
        language: Some(JsFileSource::astro().into()),
    }
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: super::ParsedSource,
    settings: &SettingsWithEditor,
) -> Result<Printed, WorkspaceError> {
    javascript::format(biome_path, document_file_source, parse, settings)
}
pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParsedSource,
    settings: &SettingsWithEditor,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    javascript::format_range(biome_path, document_file_source, parse, settings, range)
}

pub(crate) fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParsedSource,
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

fn fix_all(params: FixAllParams) -> Result<Option<FixedFileResult>, WorkspaceError> {
    javascript::fix_all(params)
}
