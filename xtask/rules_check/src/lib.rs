//! This module is in charge of checking if the documentation and tests cases inside the Analyzer rules are correct.
//!
//!
use anyhow::{bail, ensure};
use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, ControlFlow, GroupCategory, Queryable, RegistryVisitor, Rule,
    RuleCategory, RuleFilter, RuleGroup, RuleMetadata,
};
use biome_configuration::Configuration;
use biome_console::{Console, markup};
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_deserialize::json::deserialize_from_json_ast;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, Severity};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_graphql_syntax::GraphqlLanguage;
use biome_js_analyze::JsAnalyzerServices;
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage, TextSize};
use biome_json_factory::make;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonObjectValue};
use biome_module_graph::ModuleGraph;
use biome_project_layout::ProjectLayout;
use biome_rowan::AstNode;
use biome_service::projects::{ProjectKey, Projects};
use biome_service::settings::ServiceLanguage;
use biome_service::workspace::DocumentFileSource;
use biome_test_utils::get_added_paths;
use camino::Utf8PathBuf;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter, Write};
use std::slice;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
struct Errors(String);

impl Errors {
    fn style_rule_error(rule_name: impl Display) -> Self {
        Self(format!(
            "The rule '{rule_name}' that belongs to the group 'style' can't have Severity::Error. Lower down the severity or change the group.",
        ))
    }

    fn action_error(rule_name: impl Display) -> Self {
        Self(format!(
            "The rule '{rule_name}' is an action, and it must have Severity::Information. Lower down the severity.",
        ))
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl std::error::Error for Errors {}

type Data = BTreeMap<&'static str, (RuleMetadata, RuleCategory)>;
pub fn check_rules() -> anyhow::Result<()> {
    #[derive(Default)]
    struct LintRulesVisitor {
        groups: BTreeMap<(&'static str, &'static str), Data>,
        errors: Vec<Errors>,
    }

    impl LintRulesVisitor {
        fn push_rule<R, L>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
        {
            let category = <R::Group as RuleGroup>::Category::CATEGORY;
            if !matches!(category, RuleCategory::Lint | RuleCategory::Action) {
                return;
            }
            if R::Group::NAME == "style" && R::METADATA.severity == Severity::Error {
                self.errors.push(Errors::style_rule_error(R::METADATA.name))
            } else if <R::Group as RuleGroup>::Category::CATEGORY == RuleCategory::Action
                && R::METADATA.severity != Severity::Information
            {
                self.errors.push(Errors::action_error(R::METADATA.name));
            } else {
                self.groups
                    .entry((<R::Group as RuleGroup>::NAME, R::METADATA.language))
                    .or_default()
                    .insert(R::METADATA.name, (R::METADATA, category));
            }
        }
    }

    impl RegistryVisitor<JsLanguage> for LintRulesVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>>
                + 'static,
        {
            self.push_rule::<R, <R::Query as Queryable>::Language>()
        }
    }

    impl RegistryVisitor<JsonLanguage> for LintRulesVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
                + 'static,
        {
            self.push_rule::<R, <R::Query as Queryable>::Language>()
        }
    }

    impl RegistryVisitor<CssLanguage> for LintRulesVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
                + 'static,
        {
            self.push_rule::<R, <R::Query as Queryable>::Language>()
        }
    }

    impl RegistryVisitor<GraphqlLanguage> for LintRulesVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
                + 'static,
        {
            self.push_rule::<R, <R::Query as Queryable>::Language>()
        }
    }

    let mut visitor = LintRulesVisitor::default();
    biome_js_analyze::visit_registry(&mut visitor);
    biome_json_analyze::visit_registry(&mut visitor);
    biome_css_analyze::visit_registry(&mut visitor);
    biome_graphql_analyze::visit_registry(&mut visitor);

    let LintRulesVisitor { groups, errors } = visitor;
    if !errors.is_empty() {
        for error in errors {
            eprintln!("{error}");
        }
        bail!("There are some rules that have errors.")
    }

    for ((group, _), rules) in groups {
        for (_, (meta, category)) in rules {
            parse_documentation(group, meta, category)?;
        }
    }

    Ok(())
}

struct CodeBlockTest {
    /// The language tag of this code block.
    tag: String,

    /// True if this is an invalid example that should trigger a diagnostic.
    expect_diagnostic: bool,

    /// Whether to ignore this code block.
    ignore: bool,

    /// Whether this is a block of configuration options instead
    /// of a valid/invalid code example, and if yes, how that
    /// block of configuration options should be parsed:
    options: OptionsParsingMode,

    /// Whether to use the last code block that was marked with
    /// `options` as the configuration settings for this code block.
    use_options: bool,

    /// The given file path in the testing in memory file system if provided.
    file_path: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum OptionsParsingMode {
    /// This code block does not contain configuration options.
    #[default]
    NoOptions,

    /// This code block contains the options for a single rule only.
    RuleOptionsOnly,

    /// This code block contains JSON that adheres to the full `biome.json` schema.
    FullConfiguration,
}

impl CodeBlockTest {
    fn document_file_source(&self) -> DocumentFileSource {
        DocumentFileSource::from_extension(&self.tag)
    }
}

impl FromStr for CodeBlockTest {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        // This is based on the parsing logic for code block languages in `rustdoc`:
        // https://github.com/rust-lang/rust/blob/6ac8adad1f7d733b5b97d1df4e7f96e73a46db42/src/librustdoc/html/markdown.rs#L873
        let tokens = input
            .split([',', ' ', '\t'])
            .map(str::trim)
            .filter(|token| !token.is_empty());

        let mut test = Self {
            tag: String::new(),
            expect_diagnostic: false,
            ignore: false,
            options: OptionsParsingMode::NoOptions,
            use_options: false,
            file_path: None,
        };

        for token in tokens {
            // Handle file=path attribute to create multi-file test scenarios
            if let Some(file) = token.strip_prefix("file=") {
                if file.is_empty() {
                    bail!("The 'file' attribute must be followed by a file path");
                }

                // Normalize to absolute paths for consistent module resolution
                let path = file
                    .trim_start_matches("./")
                    .trim_start_matches("../")
                    .trim();
                test.file_path = Some(format!("/{path}"));

                continue;
            }

            match token {
                // Other attributes
                "expect_diagnostic" => test.expect_diagnostic = true,
                "ignore" => test.ignore = true,
                "options" => test.options = OptionsParsingMode::RuleOptionsOnly,
                "full_options" => test.options = OptionsParsingMode::FullConfiguration,
                "use_options" => test.use_options = true,
                // Regard as language tags, last one wins
                _ => test.tag = token.to_string(),
            }
        }

        Ok(test)
    }
}

struct DiagnosticWriter<'a> {
    group: &'a str,
    rule: &'a str,
    test: &'a CodeBlockTest,
    code: &'a str,
    diagnostic_count: i32,
    all_diagnostics: Vec<biome_diagnostics::Error>,
    has_error: bool,
    subtract_offset: TextSize,
}

impl<'a> DiagnosticWriter<'a> {
    pub fn new(group: &'a str, rule: &'a str, test: &'a CodeBlockTest, code: &'a str) -> Self {
        DiagnosticWriter {
            group,
            rule,
            test,
            code,
            diagnostic_count: 0,
            all_diagnostics: vec![],
            has_error: false,
            subtract_offset: TextSize::from(0),
        }
    }

    pub fn write_diagnostic(&mut self, diag: biome_diagnostics::Error) -> anyhow::Result<()> {
        let group = self.group;
        let rule = self.rule;
        let code = self.code;

        // Record the diagnostic
        self.all_diagnostics.push(self.adjust_span_offset(diag));

        // Fail the test if the analysis returns more diagnostics than expected...
        if self.test.expect_diagnostic {
            if self.all_diagnostics.len() > 1 {
                self.print_all_diagnostics();
                self.has_error = true;
                bail!(
                    "Analysis of '{group}/{rule}' on the following code block returned multiple diagnostics.\n\n{code}"
                );
            }
        } else {
            // ...or if the analysis returns a diagnostic when it is expected to not report one.
            self.print_all_diagnostics();
            self.has_error = true;
            bail!(
                "Analysis of '{group}/{rule}' on the following code block returned an unexpected diagnostic.\n\n{code}"
            );
        }
        self.diagnostic_count += 1;
        Ok(())
    }

    /// Prints all diagnostics to help the user.
    fn print_all_diagnostics(&mut self) {
        let mut console = biome_console::EnvConsole::default();
        for diag in self.all_diagnostics.iter() {
            console.println(
                biome_console::LogLevel::Error,
                markup! {
                    {PrintDiagnostic::verbose(diag)}
                },
            );
        }
    }

    /// Adjusts the location of the diagnostic to account for synthetic nodes
    /// that arent't present in the source code but only in the AST.
    fn adjust_span_offset(&self, diag: biome_diagnostics::Error) -> biome_diagnostics::Error {
        if self.subtract_offset != 0.into() {
            if let Some(span) = diag.location().span {
                let new_span = span.checked_sub(self.subtract_offset);
                diag.with_file_span(new_span)
            } else {
                diag
            }
        } else {
            diag
        }
    }
}

fn create_analyzer_options<L>(
    workspace_settings: &Projects,
    project_key: ProjectKey,
    file_path: &String,
    test: &CodeBlockTest,
) -> AnalyzerOptions
where
    L: ServiceLanguage,
{
    let path = BiomePath::new(Utf8PathBuf::from(&file_path));
    let file_source = &test.document_file_source();
    let suppression_reason = None;

    let Some(settings) = workspace_settings.get_root_settings(project_key) else {
        return AnalyzerOptions::default();
    };
    let language_settings = &L::lookup_settings(&settings.languages).linter;

    let environment = L::resolve_environment(&settings);

    L::resolve_analyzer_options(
        &settings,
        language_settings,
        environment,
        &path,
        file_source,
        suppression_reason,
    )
}

/// Parse and analyze the provided code block, and asserts that it emits
/// exactly zero or one diagnostic depending on the value of `expect_diagnostic`.
/// That diagnostic is then emitted as text into the `content` buffer
fn assert_lint(
    group: &'static str,
    rule: &'static str,
    test: &CodeBlockTest,
    code: &str,
    config: &Option<Configuration>,
    test_files: &HashMap<String, String>,
) -> anyhow::Result<()> {
    let file_path = test
        .file_path
        .clone()
        .unwrap_or_else(|| format!("code-block.{}", test.tag));

    if test.ignore {
        return Ok(());
    }

    // Record the diagnostics emitted by the lint rule to later check if
    // what was emitted matches the expectations set for this code block.
    let mut diagnostics = DiagnosticWriter::new(group, rule, test, code);

    // Create a synthetic workspace configuration
    let workspace_settings = Projects::default();
    let project_key = workspace_settings.insert_project(Utf8PathBuf::new());

    // Load settings from the preceding `json,options` block if requested
    if test.use_options {
        let Some(partial_config) = config else {
            bail!(
                "Code blocks tagged with 'use_options' must be preceded by a valid 'json,options' code block."
            );
        };

        if let Some(mut settings) = workspace_settings.get_root_settings(project_key) {
            settings.merge_with_configuration(partial_config.clone(), None)?;
            workspace_settings.set_root_settings(project_key, settings);
        }
    }

    match test.document_file_source() {
        DocumentFileSource::Js(file_source) => {
            // Temporary support for astro, svelte and vue code blocks
            let (code, file_source) = match file_source.as_embedding_kind() {
                EmbeddingKind::Astro => (
                    biome_service::file_handlers::AstroFileHandler::input(code),
                    JsFileSource::ts(),
                ),
                EmbeddingKind::Svelte => (
                    biome_service::file_handlers::SvelteFileHandler::input(code),
                    biome_service::file_handlers::SvelteFileHandler::file_source(code),
                ),
                EmbeddingKind::Vue => (
                    biome_service::file_handlers::VueFileHandler::input(code),
                    biome_service::file_handlers::VueFileHandler::file_source(code),
                ),
                _ => (code, file_source),
            };

            let parse = biome_js_parser::parse(code, file_source, JsParserOptions::default());

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag.with_file_path(&file_path).with_file_source_code(code);
                    diagnostics.write_diagnostic(error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = create_analyzer_options::<JsLanguage>(
                    &workspace_settings,
                    project_key,
                    &file_path,
                    test,
                );

                let services = get_test_services(file_source, test_files);

                biome_js_analyze::analyze(&root, filter, &options, &[], services, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag.with_file_path(&file_path).with_file_source_code(code);
                        let res = diagnostics.write_diagnostic(error);

                        // Abort the analysis on error
                        if let Err(err) = res {
                            eprintln!("Error: {err}");
                            return ControlFlow::Break(err);
                        }
                    }

                    ControlFlow::Continue(())
                });
            }
        }
        DocumentFileSource::Json(file_source) => {
            let parse = biome_json_parser::parse_json(code, JsonParserOptions::from(&file_source));

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag.with_file_path(&file_path).with_file_source_code(code);
                    diagnostics.write_diagnostic(error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = create_analyzer_options::<JsonLanguage>(
                    &workspace_settings,
                    project_key,
                    &file_path,
                    test,
                );

                biome_json_analyze::analyze(&root, filter, &options, file_source, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag.with_file_path(&file_path).with_file_source_code(code);
                        let res = diagnostics.write_diagnostic(error);

                        // Abort the analysis on error
                        if let Err(err) = res {
                            eprintln!("Error: {err}");
                            return ControlFlow::Break(err);
                        }
                    }

                    ControlFlow::Continue(())
                });
            }
        }
        DocumentFileSource::Css(..) => {
            let parse = biome_css_parser::parse_css(code, CssParserOptions::default());

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag.with_file_path(&file_path).with_file_source_code(code);
                    diagnostics.write_diagnostic(error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = create_analyzer_options::<JsonLanguage>(
                    &workspace_settings,
                    project_key,
                    &file_path,
                    test,
                );

                biome_css_analyze::analyze(&root, filter, &options, &[], |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag.with_file_path(&file_path).with_file_source_code(code);
                        let res = diagnostics.write_diagnostic(error);

                        // Abort the analysis on error
                        if let Err(err) = res {
                            eprintln!("Error: {err}");
                            return ControlFlow::Break(err);
                        }
                    }

                    ControlFlow::Continue(())
                });
            }
        }
        DocumentFileSource::Graphql(..) => {
            let parse = biome_graphql_parser::parse_graphql(code);

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag.with_file_path(&file_path).with_file_source_code(code);
                    diagnostics.write_diagnostic(error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = create_analyzer_options::<JsonLanguage>(
                    &workspace_settings,
                    project_key,
                    &file_path,
                    test,
                );

                biome_graphql_analyze::analyze(&root, filter, &options, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag.with_file_path(&file_path).with_file_source_code(code);
                        let res = diagnostics.write_diagnostic(error);

                        // Abort the analysis on error
                        if let Err(err) = res {
                            eprintln!("Error: {err}");
                            return ControlFlow::Break(err);
                        }
                    }

                    ControlFlow::Continue(())
                });
            }
        }
        DocumentFileSource::Html(..) => todo!("HTML analysis is not yet supported"),
        DocumentFileSource::Grit(..) => todo!("Grit analysis is not yet supported"),

        // Unknown code blocks should be ignored by tests
        DocumentFileSource::Unknown | DocumentFileSource::Ignore => {}
    }

    if test.expect_diagnostic {
        // Fail the test if the analysis didn't emit any diagnostic
        ensure!(
            diagnostics.diagnostic_count == 1,
            "Analysis of '{group}/{rule}' on the following code block with path '{file_path}' returned no diagnostics.\n\n{code}",
        );
    }

    if diagnostics.has_error {
        bail!(
            "A code snippet must emit one single diagnostic, but it seems multiple diagnostics were emitted. Make sure that all the snippets inside the code block 'expect_diagnostic' emit only one diagnostic."
        )
    }

    Ok(())
}

/// Creates a synthetic JSON AST for an object literal with a single member.
fn make_json_object_with_single_member<V: Into<AnyJsonValue>>(
    name: &str,
    value: V,
) -> JsonObjectValue {
    make::json_object_value(
        make::token(biome_json_syntax::JsonSyntaxKind::L_CURLY),
        make::json_member_list(
            [make::json_member(
                make::json_member_name(make::json_string_literal(name)),
                make::token(biome_json_syntax::JsonSyntaxKind::COLON),
                value.into(),
            )],
            [],
        ),
        make::token(biome_json_syntax::JsonSyntaxKind::R_CURLY),
    )
}

fn get_first_member<V: Into<AnyJsonValue>>(parent: V, expected_name: &str) -> Option<AnyJsonValue> {
    let parent_value: AnyJsonValue = parent.into();
    let member = parent_value
        .as_json_object_value()?
        .json_member_list()
        .into_iter()
        .next()?
        .ok()?;
    let member_name = member.name().ok()?.inner_string_text().ok()?.to_string();

    if member_name.as_str() == expected_name {
        member.value().ok()
    } else {
        None
    }
}

/// Parse the options fragment for a lint rule and return the parsed options.
fn parse_rule_options(
    group: &'static str,
    rule_metadata: &RuleMetadata,
    category: RuleCategory,
    test: &CodeBlockTest,
    code: &str,
) -> anyhow::Result<Option<Configuration>> {
    let file_path = format!("code-block.{}", test.tag);

    // Record the diagnostics emitted during configuration parsing to later check
    // if what was emitted matches the expectations set for this code block.
    let mut diagnostics = DiagnosticWriter::new(group, rule_metadata.name, test, code);

    match test.document_file_source() {
        DocumentFileSource::Json(file_source) => {
            let parse = biome_json_parser::parse_json(code, JsonParserOptions::from(&file_source));

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag.with_file_path(&file_path).with_file_source_code(code);
                    diagnostics.write_diagnostic(error)?;
                }
                // Parsing failed, but test.expect_diagnostic is true
                return Ok(None);
            }

            let parsed_root = parse.tree();
            let parsed_options = parsed_root.value()?;

            let root = match test.options {
                OptionsParsingMode::NoOptions => {
                    unreachable!("parse_rule_options should only be called for options blocks")
                }
                OptionsParsingMode::RuleOptionsOnly => {
                    // By convention, the configuration blocks in the documentation
                    // only contain the settings for the lint rule itself, like so:
                    //
                    // ```json,options
                    // {
                    //     "options": {
                    //         ...
                    //     }
                    // }
                    // ```
                    //
                    // We therefore extend the JSON AST with some synthetic elements
                    // to make it match the structure expected by the configuration parse:
                    //
                    // {
                    //     "linter": {
                    //         "rules": {
                    //             "<group>": {
                    //                 "<rule>": {<options>}
                    //             }
                    //         }
                    //     }
                    // }
                    let lint_or_assist = if category == RuleCategory::Lint {
                        "linter"
                    } else {
                        "assist"
                    };
                    let rules_or_actions = if category == RuleCategory::Lint {
                        "rules"
                    } else {
                        "actions"
                    };
                    let synthetic_tree = make_json_object_with_single_member(
                        lint_or_assist,
                        make_json_object_with_single_member(
                            rules_or_actions,
                            make_json_object_with_single_member(
                                group,
                                make_json_object_with_single_member(
                                    rule_metadata.name,
                                    parsed_options,
                                ),
                            ),
                        ),
                    );

                    // Create a new JsonRoot from the synthetic AST
                    let eof_token = parsed_root.eof_token()?;
                    let mut root_builder = make::json_root(synthetic_tree.into(), eof_token);
                    if let Some(bom_token) = parsed_root.bom_token() {
                        root_builder = root_builder.with_bom_token(bom_token);
                    }
                    let synthetic_root = root_builder.build();

                    // Adjust source code spans to account for the synthetic nodes
                    // so that errors are reported at the correct source code locations:
                    let original_offset =
                        parsed_root.value().ok().map(|v| AstNode::range(&v).start());
                    let wrapped_offset = synthetic_root
                        .value()
                        .ok()
                        .and_then(|v| get_first_member(v, lint_or_assist))
                        .and_then(|v| get_first_member(v, rules_or_actions))
                        .and_then(|v| get_first_member(v, group))
                        .and_then(|v| get_first_member(v, rule_metadata.name))
                        .map(|v| AstNode::range(&v).start());
                    diagnostics.subtract_offset = wrapped_offset
                        .zip(original_offset)
                        .and_then(|(wrapped, original)| wrapped.checked_sub(original))
                        .unwrap_or_default();

                    synthetic_root
                }
                OptionsParsingMode::FullConfiguration => {
                    // In some rare cases, we want to be able to display full JSON configuration
                    // instead, e.t. to be able to show off per-file overrides:
                    //
                    // ```json,full-options
                    // {
                    //     "linter": {
                    //         "rules": {
                    //             "<group>": {
                    //                 "<rule>": {<options>}
                    //             }
                    //         }
                    //     }
                    // }
                    // ```
                    parsed_root
                }
            };

            // Deserialize the configuration from the partially-synthetic AST,
            // and report any errors encountered during deserialization.
            let deserialized = deserialize_from_json_ast::<Configuration>(&root, "");
            let (partial_configuration, deserialize_diagnostics) = deserialized.consume();

            if !deserialize_diagnostics.is_empty() {
                for diag in deserialize_diagnostics {
                    let error = diag.with_file_path(&file_path).with_file_source_code(code);
                    diagnostics.write_diagnostic(error)?;
                }
                // Deserialization failed, but test.expect_diagnostic is true
                return Ok(None);
            }

            let Some(result) = partial_configuration else {
                bail!(
                    "Failed to deserialize configuration options for '{group}/{}' from the following code block due to unknown error.\n\n{code}",
                    rule_metadata.name
                );
            };

            Ok(Some(result))
        }
        _ => {
            // Only JSON code blocks can contain configuration options
            bail!(
                "The following non-JSON code block for '{group}/{}' was marked as containing configuration options. Only JSON code blocks can used to provide configuration options.\n\n{code}",
                rule_metadata.name
            );
        }
    }
}

/// Parse the documentation fragment for a lint rule (in markdown) and lint the code blocks.
fn parse_documentation(
    group: &'static str,
    rule_metadata: RuleMetadata,
    category: RuleCategory,
) -> anyhow::Result<()> {
    let parser = Parser::new(rule_metadata.docs);

    let mut test_runner = TestRunner::new(group, rule_metadata.name);

    // Track the last configuration options block that was encountered
    let mut last_options: Option<Configuration> = None;

    // Tracks the content of the current code block if it's using a
    // language supported for analysis
    let mut language = None;
    for event in parser {
        match event {
            // CodeBlock-specific handling
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(meta))) => {
                // Track the content of code blocks to pass them through the analyzer
                let test = CodeBlockTest::from_str(meta.as_ref())?;
                language = Some((test, String::new()));
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((test, block)) = language.take() {
                    if test.options != OptionsParsingMode::NoOptions {
                        last_options =
                            parse_rule_options(group, &rule_metadata, category, &test, &block)?;
                    } else {
                        if let Some(file_path) = &test.file_path {
                            test_runner
                                .file_system
                                .insert(file_path.clone(), block.clone());
                        }

                        test_runner.pending_tests.push(PendingTest {
                            test,
                            block,
                            options_snapshot: last_options.clone(),
                        });
                    }
                }
            }
            Event::Text(text) => {
                if let Some((_, block)) = &mut language {
                    if let Some(inner_text) = text.strip_prefix("# ") {
                        // Lines prefixed with "# " are hidden from the public documentation
                        write!(block, "{inner_text}")?;
                    } else {
                        write!(block, "{text}")?;
                    }
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                // Major headings delineate testable sections. When we encounter a new section,
                // run all tests from the previous section with the complete file system.
                if matches!(
                    level,
                    HeadingLevel::H1 | HeadingLevel::H2 | HeadingLevel::H3 | HeadingLevel::H4
                ) {
                    test_runner.run_pending_tests()?;
                }
            }
            // We don't care other events
            _ => {}
        }
    }

    test_runner.run_pending_tests()?;

    Ok(())
}

struct PendingTest {
    /// The test definition
    test: CodeBlockTest,
    /// The code block content for the test
    block: String,
    /// The last encountered configuration options block seen before this test was collected.
    /// We take a copy of the options because one document may contain multiple options blocks.
    options_snapshot: Option<Configuration>,
}

/// The test runner collects code block tests into batches grouped by documentation sections
/// (delineated by markdown headings). It gathers all context required for each test,
/// including options and in-memory files that may be referenced by the code blocks.
struct TestRunner {
    group: &'static str,
    rule_name: &'static str,

    /// Code block tests for the current documentation section.
    /// Tests are deferred and run as a batch when the section ends.
    pub pending_tests: Vec<PendingTest>,

    /// In-memory file system for code blocks annotated with `file=path`.
    /// All files are collected before running tests, ensuring each test
    /// has access to the complete file system regardless of definition order.
    /// This is essential for multi-file rules like import cycle detection.
    pub file_system: HashMap<String, String>,
}

impl TestRunner {
    pub fn new(group: &'static str, rule_name: &'static str) -> Self {
        Self {
            group,
            rule_name,
            pending_tests: Vec::new(),
            file_system: HashMap::new(),
        }
    }

    /// Runs all pending tests with the current file system, then resets state for the next section.
    pub fn run_pending_tests(&mut self) -> anyhow::Result<()> {
        for test in &self.pending_tests {
            assert_lint(
                self.group,
                self.rule_name,
                &test.test,
                &test.block,
                &test.options_snapshot,
                &self.file_system,
            )?;
        }

        self.pending_tests.clear();
        self.file_system.clear();

        Ok(())
    }
}

/// Creates an in-memory module graph for the given files.
/// Returns an empty module graph if no files are provided.
fn get_test_services(
    file_source: JsFileSource,
    files: &HashMap<String, String>,
) -> JsAnalyzerServices {
    if files.is_empty() {
        return JsAnalyzerServices::from((Default::default(), Default::default(), file_source));
    }

    let fs = MemoryFileSystem::default();
    let layout = ProjectLayout::default();

    let mut added_paths = Vec::with_capacity(files.len());

    for (path, src) in files.iter() {
        let path_buf = Utf8PathBuf::from(path);
        let biome_path = BiomePath::new(&path_buf);
        if biome_path.is_manifest() {
            match biome_path.file_name() {
                Some("package.json") => {
                    let parsed = parse_json(src, JsonParserOptions::default());
                    layout.insert_serialized_node_manifest(
                        path_buf.parent().unwrap().into(),
                        &parsed.syntax().as_send().unwrap(),
                    );
                }
                Some("tsconfig.json") => {
                    let parsed = parse_json(
                        src,
                        JsonParserOptions::default()
                            .with_allow_comments()
                            .with_allow_trailing_commas(),
                    );
                    layout.insert_serialized_tsconfig(
                        path_buf.parent().unwrap().into(),
                        &parsed.syntax().as_send().unwrap(),
                    );
                }
                _ => unimplemented!("Unhandled manifest: {biome_path}"),
            }
        } else {
            added_paths.push(biome_path);
        }

        fs.insert(path_buf, src.as_bytes().to_vec());
    }

    let module_graph = ModuleGraph::default();
    let added_paths = get_added_paths(&fs, &added_paths);
    module_graph.update_graph_for_js_paths(&fs, &layout, &added_paths, &[]);

    JsAnalyzerServices::from((Arc::new(module_graph), Arc::new(layout), file_source))
}
