use super::{
    AnalyzerVisitorBuilder, CodeActionsParams, EnabledForPath, ExtensionHandler, FixAllParams,
    LintParams, LintResults, ParseResult, ProcessFixAll, ProcessLint, SearchCapabilities, search,
};
use crate::WorkspaceError;
use crate::configuration::to_analyzer_rules;
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FormatterCapabilities, ParserCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, OverrideSettings, ServiceLanguage,
    Settings, SettingsWithEditor, check_feature_activity, check_override_feature_activity,
};
use crate::workspace::{
    CodeAction, DocumentFileSource, FixFileResult, GetSyntaxTreeResult, PullActionsResult,
};
use biome_analyze::options::PreferredQuote;
use biome_analyze::{AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never};
use biome_configuration::css::{
    CssAllowWrongLineCommentsEnabled, CssAssistConfiguration, CssAssistEnabled,
    CssFormatterConfiguration, CssFormatterEnabled, CssLinterConfiguration, CssLinterEnabled,
    CssModulesEnabled, CssParserConfiguration, CssTailwindDirectivesEnabled,
};
use biome_css_analyze::{CssAnalyzerServices, analyze};
use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node;
use biome_css_parser::{CssModulesKind, CssParserOptions};
use biome_css_semantic::semantic_model;
use biome_css_syntax::{AnyCssRoot, CssLanguage, CssRoot, CssSyntaxNode};
use biome_formatter::{
    FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, QuoteStyle,
    TrailingNewline,
};
use biome_fs::BiomePath;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache};
use biome_rowan::{TextRange, TextSize, TokenAtOffset};
use camino::Utf8Path;
use either::Either;
use std::borrow::Cow;
use tracing::{error, info};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub quote_style: Option<QuoteStyle>,
    pub enabled: Option<CssFormatterEnabled>,
    pub trailing_newline: Option<TrailingNewline>,
}

impl From<CssFormatterConfiguration> for CssFormatterSettings {
    fn from(configuration: CssFormatterConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
            line_width: configuration.line_width,
            indent_width: configuration.indent_width,
            indent_style: configuration.indent_style,
            quote_style: configuration.quote_style,
            line_ending: configuration.line_ending,
            trailing_newline: configuration.trailing_newline,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssLinterSettings {
    pub enabled: Option<CssLinterEnabled>,
    pub suppression_reason: Option<String>,
}

impl From<CssLinterConfiguration> for CssLinterSettings {
    fn from(value: CssLinterConfiguration) -> Self {
        Self {
            enabled: value.enabled,
            suppression_reason: None,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssAssistSettings {
    pub enabled: Option<CssAssistEnabled>,
}

impl From<CssAssistConfiguration> for CssAssistSettings {
    fn from(configuration: CssAssistConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssParserSettings {
    pub allow_wrong_line_comments: Option<CssAllowWrongLineCommentsEnabled>,
    pub css_modules_enabled: Option<CssModulesEnabled>,
    pub tailwind_directives: Option<CssTailwindDirectivesEnabled>,
}

impl From<CssParserConfiguration> for CssParserSettings {
    fn from(configuration: CssParserConfiguration) -> Self {
        Self {
            allow_wrong_line_comments: configuration.allow_wrong_line_comments,
            css_modules_enabled: configuration.css_modules,
            tailwind_directives: configuration.tailwind_directives,
        }
    }
}

impl CssFormatterSettings {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

impl CssParserSettings {
    pub fn css_modules_enabled(&self) -> bool {
        self.css_modules_enabled.unwrap_or_default().into()
    }

    pub fn allow_wrong_line_comments(&self) -> bool {
        self.allow_wrong_line_comments.unwrap_or_default().into()
    }

    pub fn tailwind_directives_enabled(&self) -> bool {
        self.tailwind_directives.unwrap_or_default().into()
    }
}

impl ServiceLanguage for CssLanguage {
    type FormatterSettings = CssFormatterSettings;
    type LinterSettings = CssLinterSettings;
    type AssistSettings = CssAssistSettings;
    type FormatOptions = CssFormatOptions;
    type ParserSettings = CssParserSettings;
    type ParserOptions = CssParserOptions;

    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.css
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        None
    }

    fn resolve_parse_options(
        overrides: &OverrideSettings,
        language: &Self::ParserSettings,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        let mut options = CssParserOptions {
            allow_wrong_line_comments: language
                .allow_wrong_line_comments
                .unwrap_or_default()
                .into(),
            css_modules: language
                .css_modules_enabled
                .map(|bool| {
                    if bool.value() {
                        CssModulesKind::Classic
                    } else {
                        CssModulesKind::None
                    }
                })
                .or_else(|| {
                    file_source.to_css_file_source().map(|files_source| {
                        if files_source.is_vue_embedded() {
                            CssModulesKind::Vue
                        } else {
                            CssModulesKind::Classic
                        }
                    })
                })
                .unwrap_or_default(),
            grit_metavariables: false,
            tailwind_directives: language.tailwind_directives.unwrap_or_default().into(),
        };

        overrides.apply_override_css_parser_options(path, &mut options);

        options
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
        document_file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        let indent_style = language
            .indent_style
            .or(global.indent_style)
            .unwrap_or_default();
        let line_width = language
            .line_width
            .or(global.line_width)
            .unwrap_or_default();
        let indent_width = language
            .indent_width
            .or(global.indent_width)
            .unwrap_or_default();

        let line_ending = language
            .line_ending
            .or(global.line_ending)
            .unwrap_or_default();

        let trailing_newline = language
            .trailing_newline
            .or(global.trailing_newline)
            .unwrap_or_default();

        let mut options = CssFormatOptions::new(
            document_file_source
                .to_css_file_source()
                .unwrap_or_default(),
        )
        .with_indent_style(indent_style)
        .with_indent_width(indent_width)
        .with_line_width(line_width)
        .with_line_ending(line_ending)
        .with_quote_style(language.quote_style.unwrap_or_default())
        .with_trailing_newline(trailing_newline);

        overrides.apply_override_css_format_options(path, &mut options);

        options
    }

    fn resolve_analyzer_options(
        global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        file_path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
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

        let configuration = AnalyzerConfiguration::default()
            .with_rules(to_analyzer_rules(global, file_path.as_path()))
            .with_preferred_quote(preferred_quote);

        AnalyzerOptions::default()
            .with_file_path(file_path.as_path())
            .with_configuration(configuration)
            .with_suppression_reason(suppression_reason)
    }

    fn linter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.css.linter.enabled,
                        pattern.linter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.css.linter.enabled,
                settings.linter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn formatter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.css.formatter.enabled,
                        pattern.formatter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.css.formatter.enabled,
                settings.formatter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn assist_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.css.assist.enabled,
                        pattern.assist.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.css.assist.enabled,
                settings.assist.enabled,
            ))
            .unwrap_or_default()
            .into()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct CssFileHandler;

impl ExtensionHandler for CssFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities {
                parse: Some(parse),
                parse_embedded_nodes: None,
            },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: Some(debug_formatter_ir),
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: Some(debug_semantic_model),
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
            search: SearchCapabilities {
                search: Some(search),
            },
            enabled_for_path: EnabledForPath {
                formatter: Some(formatter_enabled),
                linter: Some(linter_enabled),
                assist: Some(assist_enabled),
                search: Some(search_enabled),
            },
        }
    }
}

fn formatter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.formatter_enabled_for_file_path::<CssLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.linter_enabled_for_file_path::<CssLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.assist_enabled_for_file_path::<CssLanguage>(path)
}

fn search_enabled(_path: &Utf8Path, _settings: &SettingsWithEditor) -> bool {
    true
}

fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: &SettingsWithEditor,
    cache: &mut NodeCache,
) -> ParseResult {
    let options = settings.parse_options::<CssLanguage>(biome_path, &file_source);

    let source_type = file_source.to_css_file_source().unwrap_or_default();
    let parse = biome_css_parser::parse_css_with_cache(text, source_type, cache, options);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
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
    settings: &SettingsWithEditor,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

fn debug_semantic_model(_path: &BiomePath, parse: AnyParse) -> Result<String, WorkspaceError> {
    let tree: AnyCssRoot = parse.tree();
    let model = semantic_model(&tree);
    Ok(model.to_string())
}

#[tracing::instrument(level = "debug", skip(parse))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

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
    settings: &SettingsWithEditor,
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
    settings: &SettingsWithEditor,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();

    let range = tree.text_range_with_trivia();
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
    let Some(file_source) = params.language.to_css_file_source() else {
        return LintResults {
            diagnostics: vec![],
            errors: 0,
            skipped_diagnostics: 0,
        };
    };

    let settings = &params.settings;
    let analyzer_options = settings.analyzer_options::<CssLanguage>(
        params.path,
        &params.language,
        params.suppression_reason.as_deref(),
    );
    let tree = params.parse.tree();

    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings.as_ref(), analyzer_options)
            .with_only(params.only)
            .with_skip(params.skip)
            .with_path(params.path.as_path())
            .with_enabled_selectors(params.enabled_selectors)
            .with_project_layout(params.project_layout.clone())
            .finish();

    let filter = AnalysisFilter {
        categories: params.categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let mut process_lint = ProcessLint::new(&params);
    let css_services = CssAnalyzerServices {
        semantic_model: params.snippet_services.and_then(|s| {
            s.as_css_services()
                .and_then(|services| services.semantic_model.as_ref())
        }),
        file_source,
    };
    let (_, analyze_diagnostics) = analyze(
        &tree,
        filter,
        &analyzer_options,
        css_services,
        &params.plugins,
        |signal| process_lint.process_signal(signal),
    );

    process_lint.into_result(
        params
            .parse
            .into_serde_diagnostics(params.diagnostic_offset),
        analyze_diagnostics,
    )
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        settings,
        path,
        module_graph: _,
        project_layout,
        language,
        only,
        skip,
        enabled_rules: rules,
        suppression_reason,
        plugins,
        categories,
        action_offset,
        document_services,
    } = params;
    let tree = parse.tree();
    let Some(file_source) = language.to_css_file_source() else {
        error!("Could not determine the file source of the file");
        return PullActionsResult {
            actions: Vec::new(),
        };
    };

    let analyzer_options =
        settings.analyzer_options::<CssLanguage>(path, &language, suppression_reason.as_deref());
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings.as_ref(), analyzer_options)
            .with_only(only)
            .with_skip(skip)
            .with_path(path.as_path())
            .with_enabled_selectors(rules)
            .with_project_layout(project_layout)
            .finish();

    let filter = AnalysisFilter {
        categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range,
    };

    info!("CSS runs the analyzer");
    let css_services = CssAnalyzerServices {
        semantic_model: document_services
            .as_css_services()
            .and_then(|services| services.semantic_model.as_ref()),
        file_source,
    };

    analyze(
        &tree,
        filter,
        &analyzer_options,
        css_services,
        &plugins,
        |signal| {
            actions.extend(signal.actions().into_code_action_iter().map(|item| {
                CodeAction {
                    category: item.category.clone(),
                    rule_name: item
                        .rule_name
                        .map(|(group, name)| (Cow::Borrowed(group), Cow::Borrowed(name))),
                    offset: action_offset,
                    suggestion: item.suggestion,
                }
            }));

            ControlFlow::<Never>::Continue(())
        },
    );

    PullActionsResult { actions }
}

/// Applies all the safe fixes to the given syntax tree.
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let mut tree: AnyCssRoot = params.parse.tree();
    let Some(file_source) = params.document_file_source.to_css_file_source() else {
        error!("Could not determine the file source of the file");
        return Ok(FixFileResult::default());
    };
    // Compute final rules (taking `overrides` into account)
    let rules = params
        .settings
        .as_ref()
        .as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<CssLanguage>(
        params.biome_path,
        &params.document_file_source,
        params.suppression_reason.as_deref(),
    );
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.settings.as_ref(), analyzer_options)
            .with_only(params.only)
            .with_skip(params.skip)
            .with_path(params.biome_path.as_path())
            .with_enabled_selectors(params.enabled_rules)
            .with_project_layout(params.project_layout.clone())
            .finish();

    let filter = AnalysisFilter {
        categories: params.rule_categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let mut process_fix_all = ProcessFixAll::new(
        &params,
        rules,
        tree.syntax().text_range_with_trivia().len().into(),
    );

    loop {
        let css_services = CssAnalyzerServices {
            semantic_model: None,
            file_source,
        };

        let (action, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            css_services,
            &params.plugins,
            |signal| process_fix_all.process_signal(signal),
        );

        let result = process_fix_all.process_action(action, |root| {
            tree = match AnyCssRoot::cast(root) {
                Some(tree) => tree,
                None => return None,
            };
            Some(tree.syntax().text_range_with_trivia().len().into())
        })?;

        if result.is_none() {
            return process_fix_all.finish(|| {
                Ok(if params.should_format {
                    Either::Left(format_node(
                        params.settings.format_options::<CssLanguage>(
                            params.biome_path,
                            &params.document_file_source,
                        ),
                        tree.syntax(),
                    ))
                } else {
                    Either::Right(tree.syntax().to_string())
                })
            });
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use biome_css_syntax::CssFileSource;

    #[test]
    fn inherit_global_format_settings() {
        let format_options = CssLanguage::resolve_format_options(
            &FormatSettings::default(),
            &OverrideSettings::default(),
            &CssFormatterSettings::default(),
            &BiomePath::new(""),
            &DocumentFileSource::Css(CssFileSource::css()),
        );
        assert_eq!(
            format_options,
            CssFormatOptions::default()
                .with_indent_style(IndentStyle::default())
                .with_indent_width(IndentWidth::default())
                .with_line_ending(LineEnding::default())
                .with_line_width(LineWidth::default())
        );
    }
}
