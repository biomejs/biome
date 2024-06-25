use std::ffi::OsStr;

use super::{CodeActionsParams, DocumentFileSource, ExtensionHandler, ParseResult};
use crate::configuration::to_analyzer_rules;
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FixAllParams, FormatterCapabilities, LintParams,
    LintResults, ParserCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, LinterSettings, OverrideSettings,
    ServiceLanguage, Settings, WorkspaceSettingsHandle,
};
use crate::workspace::{
    FixFileResult, GetSyntaxTreeResult, OrganizeImportsResult, PullActionsResult,
};
use crate::WorkspaceError;
use biome_analyze::options::PreferredQuote;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never, RuleCategory,
};
use biome_configuration::PartialConfiguration;
use biome_deserialize::json::deserialize_from_json_ast;
use biome_diagnostics::{category, Diagnostic, DiagnosticExt, Severity};
use biome_formatter::{FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed};
use biome_fs::{BiomePath, ConfigName, ROME_JSON};
use biome_json_analyze::analyze;
use biome_json_formatter::context::{JsonFormatOptions, TrailingCommas};
use biome_json_formatter::format_node;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::{JsonLanguage, JsonRoot, JsonSyntaxNode};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache};
use biome_rowan::{TextRange, TextSize, TokenAtOffset};
use tracing::{debug_span, trace_span};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub trailing_commas: Option<TrailingCommas>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonParserSettings {
    pub allow_comments: bool,
    pub allow_trailing_commas: bool,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonLinterSettings {
    pub enabled: Option<bool>,
}

impl ServiceLanguage for JsonLanguage {
    type FormatterSettings = JsonFormatterSettings;
    type LinterSettings = JsonLinterSettings;
    type OrganizeImportsSettings = ();
    type FormatOptions = JsonFormatOptions;
    type ParserSettings = JsonParserSettings;
    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.json
    }

    fn resolve_format_options(
        global: Option<&FormatSettings>,
        overrides: Option<&OverrideSettings>,
        language: Option<&JsonFormatterSettings>,
        path: &BiomePath,
        _document_file_source: &DocumentFileSource,
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

        // ensure it never formats biome.json into a form it can't parse
        let trailing_commas =
            if matches!(path.file_name().and_then(OsStr::to_str), Some("biome.json")) {
                TrailingCommas::None
            } else {
                language.and_then(|l| l.trailing_commas).unwrap_or_default()
            };

        let options = JsonFormatOptions::new()
            .with_line_ending(line_ending)
            .with_indent_style(indent_style)
            .with_indent_width(indent_width)
            .with_line_width(line_width)
            .with_trailing_commas(trailing_commas);

        if let Some(overrides) = overrides {
            overrides.to_override_json_format_options(path, options)
        } else {
            options
        }
    }

    fn resolve_analyzer_options(
        global: Option<&Settings>,
        _linter: Option<&LinterSettings>,
        _overrides: Option<&OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> AnalyzerOptions {
        let configuration = AnalyzerConfiguration {
            rules: global
                .map(|g| to_analyzer_rules(g, path.as_path()))
                .unwrap_or_default(),
            globals: vec![],
            preferred_quote: PreferredQuote::Double,
            jsx_runtime: Default::default(),
        };
        AnalyzerOptions {
            configuration,
            file_path: path.to_path_buf(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct JsonFileHandler;

impl ExtensionHandler for JsonFileHandler {
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
        }
    }
}

fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: Option<&Settings>,
    cache: &mut NodeCache,
) -> ParseResult {
    let parser = settings.map(|s| &s.languages.json.parser);
    let overrides = settings.map(|s| &s.override_settings);
    let optional_json_file_source = file_source.to_json_file_source();
    let options = JsonParserOptions {
        allow_comments: parser.map(|p| p.allow_comments).unwrap_or_default()
            || optional_json_file_source.map_or(false, |x| x.allow_comments()),
        allow_trailing_commas: parser.map(|p| p.allow_trailing_commas).unwrap_or_default()
            || optional_json_file_source.map_or(false, |x| x.allow_trailing_commas()),
    };
    let options = if let Some(overrides) = overrides {
        overrides.to_override_json_parser_options(biome_path, options)
    } else {
        options
    };
    let parse = biome_json_parser::parse_json_with_cache(text, cache, options);
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();

    ParseResult {
        any_parse: AnyParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        ),
        language: Some(file_source),
    }
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: JsonSyntaxNode = parse.syntax();
    let tree: JsonRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(path, document_file_source);

    tracing::debug!("Format with the following options: \n{}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

fn format_range(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(path, document_file_source);

    let tree = parse.syntax();
    let printed = biome_json_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(path, document_file_source);

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

    let printed = biome_json_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn lint(params: LintParams) -> LintResults {
    tracing::debug_span!("Linting JSON file", path =? params.path, language =? params.language)
        .in_scope(move || {
            let root: JsonRoot = params.parse.tree();
            let mut diagnostics = params.parse.into_diagnostics();

            // if we're parsing the `biome.json` file, we deserialize it, so we can emit diagnostics for
            // malformed configuration
            if params.path.ends_with(ROME_JSON)
                || params.path.ends_with(ConfigName::biome_json())
                || params.path.ends_with(ConfigName::biome_jsonc())
            {
                let deserialized = deserialize_from_json_ast::<PartialConfiguration>(&root, "");
                diagnostics.extend(
                    deserialized
                        .into_diagnostics()
                        .into_iter()
                        .map(biome_diagnostics::serde::Diagnostic::new)
                        .collect::<Vec<_>>(),
                );
            }

            let analyzer_options = &params
                .workspace
                .analyzer_options::<JsonLanguage>(params.path, &params.language);

            let mut rules = None;
            if let Some(settings) = params.workspace.settings() {
                rules = settings.as_rules(params.path.as_path());
            }

            let has_only_filter = !params.only.is_empty();
            let enabled_rules = if has_only_filter {
                params
                    .only
                    .into_iter()
                    .map(|selector| selector.into())
                    .collect::<Vec<_>>()
            } else {
                rules
                    .as_ref()
                    .map(|rules| rules.as_enabled_rules())
                    .unwrap_or_default()
                    .into_iter()
                    .collect::<Vec<_>>()
            };
            let disabled_rules = params
                .skip
                .into_iter()
                .map(|selector| selector.into())
                .collect::<Vec<_>>();
            let filter = AnalysisFilter {
                categories: params.categories,
                enabled_rules: Some(enabled_rules.as_slice()),
                disabled_rules: &disabled_rules,
                range: None,
            };

            // Do not report unused suppression comment diagnostics if:
            // - it is a syntax-only analyzer pass, or
            // - if a single rule is run.
            let ignores_suppression_comment =
                !filter.categories.contains(RuleCategory::Lint) || has_only_filter;

            let mut diagnostic_count = diagnostics.len() as u32;
            let mut errors = diagnostics
                .iter()
                .filter(|diag| diag.severity() <= Severity::Error)
                .count();
            let skipped_diagnostics = diagnostic_count - diagnostics.len() as u32;

            let (_, analyze_diagnostics) = analyze(&root, filter, analyzer_options, |signal| {
                if let Some(mut diagnostic) = signal.diagnostic() {
                    if ignores_suppression_comment
                        && diagnostic.category() == Some(category!("suppressions/unused"))
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

                    if severity <= Severity::Error {
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

            LintResults {
                diagnostics,
                errors,
                skipped_diagnostics,
            }
        })
}

fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        workspace: _,
        path,
        manifest: _,
        language: _,
        settings: _,
    } = params;

    debug_span!("Code actions JSON",  range =? range, path =? path).in_scope(move || {
        let tree: JsonRoot = parse.tree();
        trace_span!("Parsed file", tree =? tree).in_scope(move || PullActionsResult {
            actions: Vec::new(),
        })
    })
}

fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let tree: JsonRoot = params.parse.tree();
    Ok(FixFileResult {
        actions: vec![],
        errors: 0,
        skipped_suggested_fixes: 0,
        code: tree.syntax().to_string(),
    })
}

fn organize_imports(parse: AnyParse) -> Result<OrganizeImportsResult, WorkspaceError> {
    Ok(OrganizeImportsResult {
        code: parse.syntax::<JsonLanguage>().to_string(),
    })
}
