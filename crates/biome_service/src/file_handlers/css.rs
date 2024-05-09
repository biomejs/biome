use super::{ExtensionHandler, LintParams, LintResults, ParseResult};
use crate::configuration::to_analyzer_rules;
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FormatterCapabilities, ParserCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, LinterSettings, OverrideSettings,
    ServiceLanguage, Settings, WorkspaceSettingsHandle,
};
use crate::workspace::{DocumentFileSource, GetSyntaxTreeResult, OrganizeImportsResult};
use crate::WorkspaceError;
use biome_analyze::options::PreferredQuote;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never, RuleCategories,
};
use biome_css_analyze::analyze;
use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::{CssLanguage, CssRoot, CssSyntaxNode};
use biome_diagnostics::{category, Diagnostic, DiagnosticExt, Severity};
use biome_formatter::{
    FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, QuoteStyle,
};
use biome_fs::BiomePath;
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use biome_rowan::{TextRange, TextSize, TokenAtOffset};
use tracing::{debug_span, info};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub quote_style: Option<QuoteStyle>,
    pub enabled: Option<bool>,
}

impl Default for CssFormatterSettings {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            indent_style: Default::default(),
            indent_width: Default::default(),
            line_ending: Default::default(),
            line_width: Default::default(),
            quote_style: Default::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssLinterSettings {
    pub enabled: Option<bool>,
}

// NOTE: we want to make the linter opt-in for now
impl Default for CssLinterSettings {
    fn default() -> Self {
        Self {
            enabled: Some(false),
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssParserSettings {
    pub allow_wrong_line_comments: bool,
}

impl ServiceLanguage for CssLanguage {
    type FormatterSettings = CssFormatterSettings;
    type LinterSettings = CssLinterSettings;
    type OrganizeImportsSettings = ();
    type FormatOptions = CssFormatOptions;
    type ParserSettings = CssParserSettings;
    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.css
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
        document_file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        let indent_style = if let Some(indent_style) = language.indent_style {
            indent_style
        } else {
            global.indent_style.unwrap_or_default()
        };
        let line_width = if let Some(line_width) = language.line_width {
            line_width
        } else {
            global.line_width.unwrap_or_default()
        };
        let indent_width = if let Some(indent_width) = language.indent_width {
            indent_width
        } else {
            global.indent_width.unwrap_or_default()
        };

        overrides.to_override_css_format_options(
            path,
            CssFormatOptions::new(
                document_file_source
                    .to_css_file_source()
                    .unwrap_or_default(),
            )
            .with_indent_style(indent_style)
            .with_indent_width(indent_width)
            .with_line_width(line_width)
            .with_quote_style(language.quote_style.unwrap_or_default()),
        )
    }

    fn resolve_analyzer_options(
        global: &Settings,
        _linter: &LinterSettings,
        _overrides: &OverrideSettings,
        _language: &Self::LinterSettings,
        file_path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> AnalyzerOptions {
        let preferred_quote = global
            .languages
            .css
            .formatter
            .quote_style
            .map(|quote_style: QuoteStyle| {
                if quote_style == QuoteStyle::Single {
                    PreferredQuote::Single
                } else {
                    PreferredQuote::Double
                }
            })
            .unwrap_or_default();

        let configuration = AnalyzerConfiguration {
            rules: to_analyzer_rules(global, file_path.as_path()),
            globals: vec![],
            preferred_quote,
            jsx_runtime: None,
        };

        AnalyzerOptions {
            configuration,
            file_path: file_path.to_path_buf(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct CssFileHandler;

impl ExtensionHandler for CssFileHandler {
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
                organize_imports: Some(organize_imports),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
        }
    }
}

fn parse(
    biome_path: &BiomePath,
    _file_source: DocumentFileSource,
    text: &str,
    settings: WorkspaceSettingsHandle,
    cache: &mut NodeCache,
) -> ParseResult {
    let parser = &settings.settings().languages.css.parser;
    let overrides = &settings.settings().override_settings;
    let options: CssParserOptions = overrides.to_override_css_parser_options(
        biome_path,
        CssParserOptions {
            allow_wrong_line_comments: parser.allow_wrong_line_comments,
        },
    );
    let parse = biome_css_parser::parse_css_with_cache(text, cache, options);
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();
    ParseResult {
        any_parse: AnyParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        ),
        language: None,
    }
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: CssSyntaxNode = parse.syntax();
    let tree: CssRoot = parse.tree();
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
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    tracing::debug!("Format with the following options: \n{}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let printed = biome_css_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

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

    let printed = biome_css_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn lint(params: LintParams) -> LintResults {
    debug_span!("Linting JavaScript file", path =? params.path, language =? params.language)
        .in_scope(move || {
            let workspace_settings = &params.settings;
            let analyzer_options =
                workspace_settings.analyzer_options::<CssLanguage>(params.path, &params.language);
            let settings = workspace_settings.settings();
            let tree = params.parse.tree();
            let mut diagnostics = params.parse.into_diagnostics();

            // Compute final rules (taking `overrides` into account)
            let rules = settings.as_rules(params.path.as_path());
            let rule_filter_list = rules
                .as_ref()
                .map(|rules| rules.as_enabled_rules())
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();

            let mut filter = AnalysisFilter::from_enabled_rules(Some(rule_filter_list.as_slice()));
            filter.categories = params.categories;

            let mut diagnostic_count = diagnostics.len() as u32;
            let mut errors = diagnostics
                .iter()
                .filter(|diag| diag.severity() <= Severity::Error)
                .count();

            let has_lint = filter.categories.contains(RuleCategories::LINT);

            info!("Analyze file {}", params.path.display());
            let (_, analyze_diagnostics) = analyze(&tree, filter, &analyzer_options, |signal| {
                if let Some(mut diagnostic) = signal.diagnostic() {
                    // Do not report unused suppression comment diagnostics if this is a syntax-only analyzer pass
                    if !has_lint && diagnostic.category() == Some(category!("suppressions/unused"))
                    {
                        return ControlFlow::<Never>::Continue(());
                    }

                    diagnostic_count += 1;

                    // We do now check if the severity of the diagnostics should be changed.
                    // The configuration allows to change the severity of the diagnostics emitted by rules.
                    let severity = diagnostic
                        .category()
                        .filter(|category| category.name().starts_with("lint/"))
                        .map_or_else(
                            || diagnostic.severity(),
                            |category| {
                                rules
                                    .as_ref()
                                    .and_then(|rules| rules.get_severity_from_code(category))
                                    .unwrap_or(Severity::Warning)
                            },
                        );

                    if severity >= Severity::Error {
                        errors += 1;
                    }

                    if diagnostic_count <= params.max_diagnostics {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diagnostic = diagnostic.add_code_suggestion(action.into());
                            }
                        }

                        let error = diagnostic.with_severity(severity);

                        diagnostics.push(biome_diagnostics::serde::Diagnostic::new(error));
                    }
                }

                ControlFlow::<Never>::Continue(())
            });

            diagnostics.extend(
                analyze_diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new)
                    .collect::<Vec<_>>(),
            );
            let skipped_diagnostics = diagnostic_count.saturating_sub(diagnostics.len() as u32);

            LintResults {
                diagnostics,
                errors,
                skipped_diagnostics,
            }
        })
}

fn organize_imports(parse: AnyParse) -> Result<OrganizeImportsResult, WorkspaceError> {
    Ok(OrganizeImportsResult {
        code: parse.syntax::<CssLanguage>().to_string(),
    })
}
