use super::{CodeActionsParams, ExtensionHandler, Mime, ParseResult};
use crate::configuration::{to_analyzer_rules, PartialConfiguration};
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FixAllParams, FormatterCapabilities, LintParams,
    LintResults, ParserCapabilities,
};
use crate::file_handlers::{DebugCapabilities, Language as LanguageId};
use crate::settings::{
    FormatSettings, Language, LanguageListSettings, LanguageSettings, OverrideSettings,
    SettingsHandle,
};
use crate::workspace::{
    FixFileResult, GetSyntaxTreeResult, OrganizeImportsResult, PullActionsResult,
};
use crate::WorkspaceError;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never, RuleCategories,
};
use biome_deserialize::json::deserialize_from_json_ast;
use biome_diagnostics::{category, Diagnostic, DiagnosticExt, Severity};
use biome_formatter::{FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed};
use biome_fs::{BiomePath, ConfigName, ROME_JSON};
use biome_json_analyze::analyze;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_formatter::format_node;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::{JsonFileSource, JsonLanguage, JsonRoot, JsonSyntaxNode};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache};
use biome_rowan::{TextRange, TextSize, TokenAtOffset};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonParserSettings {
    pub allow_comments: bool,
    pub allow_trailing_commas: bool,
}

impl Language for JsonLanguage {
    type FormatterSettings = JsonFormatterSettings;
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = JsonFormatOptions;
    type ParserSettings = JsonParserSettings;
    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.json
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
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

        let line_ending = if let Some(line_ending) = language.line_ending {
            line_ending
        } else {
            global.line_ending.unwrap_or_default()
        };

        overrides.override_json_format_options(
            path,
            JsonFormatOptions::new(path.as_path().try_into().unwrap_or_default())
                .with_line_ending(line_ending)
                .with_indent_style(indent_style)
                .with_indent_width(indent_width)
                .with_line_width(line_width),
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct JsonFileHandler;

impl ExtensionHandler for JsonFileHandler {
    fn language(&self) -> super::Language {
        super::Language::Json
    }

    fn mime(&self) -> super::Mime {
        Mime::Json
    }

    fn may_use_tabs(&self) -> bool {
        true
    }

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

fn is_file_allowed(path: &Path) -> bool {
    path.file_name()
        .and_then(|f| f.to_str())
        .map(|f| {
            super::Language::KNOWN_FILES_AS_JSONC
                .binary_search(&f)
                .is_ok()
        })
        // default is false
        .unwrap_or_default()
}

fn parse(
    biome_path: &BiomePath,
    language_hint: LanguageId,
    text: &str,
    settings: SettingsHandle,
    cache: &mut NodeCache,
) -> ParseResult {
    let parser = &settings.as_ref().languages.json.parser;
    let overrides = &settings.as_ref().override_settings;
    let source_type =
        JsonFileSource::try_from(biome_path.as_path()).unwrap_or_else(|_| match language_hint {
            LanguageId::Json => JsonFileSource::json(),
            LanguageId::Jsonc => JsonFileSource::jsonc(),
            _ => JsonFileSource::json(),
        });
    let options: JsonParserOptions = overrides.override_json_parser_options(
        biome_path,
        JsonParserOptions {
            allow_comments: parser.allow_comments
                || source_type.is_jsonc()
                || is_file_allowed(biome_path),
            allow_trailing_commas: parser.allow_trailing_commas || is_file_allowed(biome_path),
        },
    );
    let parse = biome_json_parser::parse_json_with_cache(text, cache, options);
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
    let syntax: JsonSyntaxNode = parse.syntax();
    let tree: JsonRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    biome_path: &BiomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(biome_path);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(biome_path);

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
    parse: AnyParse,
    settings: SettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(biome_path);

    let tree = parse.syntax();
    let printed = biome_json_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    biome_path: &BiomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(biome_path);

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
            let settings = params.settings.as_ref();

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

            let mut diagnostic_count = diagnostics.len() as u32;
            let mut errors = diagnostics
                .iter()
                .filter(|diag| diag.severity() <= Severity::Error)
                .count();

            let skipped_diagnostics = diagnostic_count - diagnostics.len() as u32;

            let rules = settings.as_rules(params.path.as_path());
            let rule_filter_list = rules
                .as_ref()
                .map(|rules| rules.as_enabled_rules())
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();

            let analyzer_options =
                compute_analyzer_options(&params.settings, PathBuf::from(params.path.as_path()));
            let mut filter = AnalysisFilter::from_enabled_rules(Some(rule_filter_list.as_slice()));
            filter.categories = params.categories;
            let has_lint = filter.categories.contains(RuleCategories::LINT);

            let (_, analyze_diagnostics) = analyze(&root, filter, &analyzer_options, |signal| {
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
                        .map(|category| {
                            rules
                                .as_ref()
                                .and_then(|rules| rules.get_severity_from_code(category))
                                .unwrap_or(Severity::Warning)
                        })
                        .unwrap_or_else(|| diagnostic.severity());

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

fn code_actions(_: CodeActionsParams) -> PullActionsResult {
    PullActionsResult {
        actions: Vec::new(),
    }
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

fn compute_analyzer_options(settings: &SettingsHandle, file_path: PathBuf) -> AnalyzerOptions {
    let configuration = AnalyzerConfiguration {
        rules: to_analyzer_rules(settings.as_ref(), file_path.as_path()),
        globals: vec![],
    };
    AnalyzerOptions {
        configuration,
        file_path,
    }
}
