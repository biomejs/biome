use super::{
    AnalyzerCapabilities, Capabilities, DebugCapabilities, DocumentFileSource, ExtensionHandler,
    FormatterCapabilities, LintParams, LintResults, ParseResult, ParserCapabilities,
    SearchCapabilities,
};
use crate::workspace::GetSyntaxTreeResult;
use crate::{
    settings::{ServiceLanguage, Settings, WorkspaceSettingsHandle},
    WorkspaceError,
};
use biome_analyze::{AnalyzerConfiguration, AnalyzerOptions};
use biome_diagnostics::{Diagnostic, Severity};
use biome_formatter::{FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed};
use biome_fs::BiomePath;
use biome_grit_formatter::{context::GritFormatOptions, format_node, format_sub_tree};
use biome_grit_parser::parse_grit_with_cache;
use biome_grit_syntax::{GritLanguage, GritRoot, GritSyntaxNode};
use biome_parser::AnyParse;
use biome_rowan::{NodeCache, TextRange, TextSize, TokenAtOffset};
use tracing::debug_span;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GritFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub enabled: Option<bool>,
}

impl Default for GritFormatterSettings {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            indent_style: Default::default(),
            indent_width: Default::default(),
            line_ending: Default::default(),
            line_width: Default::default(),
        }
    }
}

impl ServiceLanguage for GritLanguage {
    type FormatterSettings = GritFormatterSettings;
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = GritFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();
    fn lookup_settings(
        languages: &crate::settings::LanguageListSettings,
    ) -> &crate::settings::LanguageSettings<Self> {
        &languages.grit
    }

    fn resolve_format_options(
        global: Option<&crate::settings::FormatSettings>,
        overrides: Option<&crate::settings::OverrideSettings>,
        language: Option<&Self::FormatterSettings>,
        path: &biome_fs::BiomePath,
        file_source: &super::DocumentFileSource,
    ) -> Self::FormatOptions {
        let indent_style = language
            .and_then(|l| l.indent_style)
            .or(global.and_then(|g| g.indent_style))
            .unwrap_or_default();
        let line_width = language
            .and_then(|l| l.line_width)
            .or(global.and_then(|g| g.line_width))
            .unwrap_or_default();
        let indent_width = language
            .and_then(|l| l.indent_width)
            .or(global.and_then(|g| g.indent_width))
            .unwrap_or_default();

        let line_ending = language
            .and_then(|l| l.line_ending)
            .or(global.and_then(|g| g.line_ending))
            .unwrap_or_default();

        let options = GritFormatOptions::new(file_source.to_grit_file_source().unwrap_or_default())
            .with_indent_style(indent_style)
            .with_indent_width(indent_width)
            .with_line_width(line_width)
            .with_line_ending(line_ending);
        if let Some(overrides) = overrides {
            overrides.to_override_grit_format_options(path, options)
        } else {
            options
        }
    }

    fn resolve_analyzer_options(
        _global: Option<&crate::settings::Settings>,
        _linter: Option<&crate::settings::LinterSettings>,
        _overrides: Option<&crate::settings::OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
        suppression_reason: Option<String>,
    ) -> biome_analyze::AnalyzerOptions {
        AnalyzerOptions {
            configuration: AnalyzerConfiguration::default(),
            file_path: path.to_path_buf(),
            suppression_reason,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct GritFileHandler;

impl ExtensionHandler for GritFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: Some(debug_formatter_ir),
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: None,
                rename: None,
                fix_all: None,
                organize_imports: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
            search: SearchCapabilities { search: None },
        }
    }
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: Option<&Settings>,
    cache: &mut NodeCache,
) -> ParseResult {
    let parse = parse_grit_with_cache(text, cache);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: GritSyntaxNode = parse.syntax();
    let tree: GritRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<GritLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<GritLanguage>(biome_path, document_file_source);

    tracing::debug!("Format with the following options: \n{}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

#[tracing::instrument(level = "debug", skip_all)]
fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<GritLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let printed = biome_grit_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

#[tracing::instrument(level = "debug", skip_all)]
fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<GritLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();

    let range = tree.text_range();
    if offset < range.start() || offset > range.end() {
        return Err(WorkspaceError::FormatError(FormatError::RangeError {
            input: TextRange::at(offset, TextSize::from(0)),
            tree: range,
        }));
    }

    let token = match tree.token_at_offset(offset) {
        // File is empty, do nothing
        TokenAtOffset::None => panic!("empty file"),
        TokenAtOffset::Single(token) => token,
        // The cursor should be right after the closing character that was just typed,
        // select the previous token as the correct one
        TokenAtOffset::Between(token, _) => token,
    };

    let root_node = match token.parent() {
        Some(node) => node,
        None => panic!("found a token with no parent"),
    };

    let printed = format_sub_tree(options, &root_node)?;
    Ok(printed)
}

#[tracing::instrument(level = "debug", skip(params))]
fn lint(params: LintParams) -> LintResults {
    let _ = debug_span!("Linting Grit file", path =? params.path, language =? params.language)
        .entered();
    let diagnostics = params.parse.into_diagnostics();

    let diagnostic_count = diagnostics.len() as u32;
    let skipped_diagnostics = diagnostic_count.saturating_sub(diagnostics.len() as u32);
    let errors = diagnostics
        .iter()
        .filter(|diag| diag.severity() <= Severity::Error)
        .count();

    LintResults {
        diagnostics,
        errors,
        skipped_diagnostics,
    }
}
