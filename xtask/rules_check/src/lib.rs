//! This module is in charge of checking if the documentation and tests cases inside the Analyzer rules are correct.
//!
//!
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use std::{mem, slice};

use anyhow::bail;
use biome_analyze::{
    AnalysisFilter, ControlFlow, GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory,
    RuleFilter, RuleGroup, RuleMetadata,
};
use biome_configuration::Configuration;
use biome_console::{Console, markup};
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_deserialize::json::deserialize_from_json_ast;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, Severity};
use biome_graphql_syntax::GraphqlLanguage;
use biome_html_parser::HtmlParseOptions;
use biome_html_syntax::HtmlLanguage;
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage, TextSize};
use biome_json_factory::make;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonObjectValue};
use biome_rowan::AstNode;
use biome_ruledoc_utils::{AnalyzerServicesBuilder, CodeBlock, OptionsParsingMode};
use biome_service::workspace::DocumentFileSource;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};

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

    impl RegistryVisitor<HtmlLanguage> for LintRulesVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
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
    biome_html_analyze::visit_registry(&mut visitor);

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

#[derive(Default)]
struct DiagnosticWriter {
    all_diagnostics: Vec<biome_diagnostics::Error>,
    has_parse_error: bool,
    subtract_offset: TextSize,
}

impl DiagnosticWriter {
    pub fn write_diagnostic(&mut self, diag: biome_diagnostics::Error) {
        self.all_diagnostics.push(self.adjust_span_offset(diag));
    }

    pub fn write_parse_error(&mut self, diag: biome_diagnostics::Error) {
        self.has_parse_error = true;
        self.write_diagnostic(diag);
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

/// Parse and analyze the provided code block, and asserts that it emits
/// exactly zero or one diagnostic depending on the value of `expect_diagnostic`.
/// That diagnostic is then emitted as text into the `content` buffer
fn assert_lint(
    group: &'static str,
    rule: &'static str,
    test: &CodeBlock,
    code: &str,
    config: Option<Configuration>,
    services_builder: &AnalyzerServicesBuilder,
) -> anyhow::Result<()> {
    if test.ignore {
        return Ok(());
    }

    // Record the diagnostics emitted by the lint rule to later check if
    // what was emitted matches the expectations set for this code block.
    let mut diagnostics = DiagnosticWriter::default();

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
                    let error = diag
                        .with_file_path(test.file_path())
                        .with_file_source_code(code);
                    diagnostics.write_parse_error(error);
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = test.create_analyzer_options::<JsLanguage>(config)?;

                let services = services_builder.build_for_js_file_source(file_source);

                biome_js_analyze::analyze(&root, filter, &options, &[], services, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag
                            .with_file_path(test.file_path())
                            .with_file_source_code(code);
                        diagnostics.write_diagnostic(error);
                    }

                    ControlFlow::<()>::Continue(())
                });
            }
        }
        DocumentFileSource::Json(file_source) => {
            let parse = biome_json_parser::parse_json(code, JsonParserOptions::from(&file_source));

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag
                        .with_file_path(test.file_path())
                        .with_file_source_code(code);
                    diagnostics.write_parse_error(error);
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = test.create_analyzer_options::<JsonLanguage>(config)?;

                biome_json_analyze::analyze(&root, filter, &options, file_source, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag
                            .with_file_path(test.file_path())
                            .with_file_source_code(code);
                        diagnostics.write_diagnostic(error);
                    }

                    ControlFlow::<()>::Continue(())
                });
            }
        }
        DocumentFileSource::Css(..) => {
            let parse_options = CssParserOptions::default()
                .allow_css_modules()
                .allow_tailwind_directives();
            let parse = biome_css_parser::parse_css(code, parse_options);

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag
                        .with_file_path(test.file_path())
                        .with_file_source_code(code);
                    diagnostics.write_parse_error(error);
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = test.create_analyzer_options::<CssLanguage>(config)?;

                biome_css_analyze::analyze(&root, filter, &options, &[], |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag
                            .with_file_path(test.file_path())
                            .with_file_source_code(code);
                        diagnostics.write_diagnostic(error);
                    }

                    ControlFlow::<()>::Continue(())
                });
            }
        }
        DocumentFileSource::Graphql(..) => {
            let parse = biome_graphql_parser::parse_graphql(code);

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag
                        .with_file_path(test.file_path())
                        .with_file_source_code(code);
                    diagnostics.write_parse_error(error);
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = test.create_analyzer_options::<GraphqlLanguage>(config)?;

                biome_graphql_analyze::analyze(&root, filter, &options, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag
                            .with_file_path(test.file_path())
                            .with_file_source_code(code);
                        diagnostics.write_diagnostic(error);
                    }

                    ControlFlow::<()>::Continue(())
                });
            }
        }
        DocumentFileSource::Html(source) => {
            let parse = biome_html_parser::parse_html(code, HtmlParseOptions::from(&source));

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag
                        .with_file_path(test.file_path())
                        .with_file_source_code(code);
                    diagnostics.write_parse_error(error);
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = test.create_analyzer_options::<HtmlLanguage>(config)?;

                biome_html_analyze::analyze(&root, filter, &options, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag
                            .with_file_path(test.file_path())
                            .with_file_source_code(code);
                        diagnostics.write_diagnostic(error);
                    }

                    ControlFlow::<()>::Continue(())
                });
            }
        }
        DocumentFileSource::Grit(..) => todo!("Grit analysis is not yet supported"),

        // Unknown code blocks should be ignored by tests
        DocumentFileSource::Unknown | DocumentFileSource::Ignore => {}
    }

    if diagnostics.has_parse_error {
        // Fail if there is a parse error...
        diagnostics.print_all_diagnostics();
        bail!(
            "Analysis of '{group}/{rule}' on the following code block resulted in a parse error.\n\n{code}"
        );
    } else if test.expect_diagnostic {
        // ...or if the analysis does not return exactly one diagnostic...
        if diagnostics.all_diagnostics.len() != 1 {
            diagnostics.print_all_diagnostics();
            bail!(
                "Analysis of '{group}/{rule}' on the following code block returned {num_diagnostics} diagnostics, but a single diagnostic was expected.\n\n{code}",
                num_diagnostics = diagnostics.all_diagnostics.len()
            );
        }
    } else if test.expect_diff {
        // ...or there is no diff...
        if diagnostics.all_diagnostics.is_empty() {
            bail!(
                "Analysis of '{group}/{rule}' on the following code block returned no diff where one was expected.\n\n{code}",
            );
        }
    } else if !diagnostics.all_diagnostics.is_empty() {
        // ...or if the analysis returns a diagnostic when none are expected.
        diagnostics.print_all_diagnostics();
        bail!(
            "Analysis of '{group}/{rule}' on the following code block returned an unexpected diagnostic.\n\n{code}"
        );
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
    block: &CodeBlock,
    code: &str,
) -> anyhow::Result<Option<Configuration>> {
    let DocumentFileSource::Json(file_source) = block.document_file_source() else {
        bail!(
            "The following non-JSON code block for '{group}/{}' was marked as containing configuration options. Only JSON code blocks can used to provide configuration options.\n\n{code}",
            rule_metadata.name
        );
    };

    // Record the diagnostics emitted during configuration parsing to later check
    // if what was emitted matches the expectations set for this code block.
    let mut diagnostics = DiagnosticWriter::default();

    let parse = biome_json_parser::parse_json(code, JsonParserOptions::from(&file_source));

    if parse.has_errors() {
        for diag in parse.into_diagnostics() {
            let error = diag
                .with_file_path(block.file_path())
                .with_file_source_code(code);
            diagnostics.write_parse_error(error);
        }
        if block.expect_diagnostic {
            return Ok(None);
        } else {
            diagnostics.print_all_diagnostics();
            bail!("Please fix the parse errors above.");
        };
    }

    let parsed_root = parse.tree();
    let parsed_options = parsed_root.value()?;

    let root = match block.options {
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
                        make_json_object_with_single_member(rule_metadata.name, parsed_options),
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
            let original_offset = parsed_root.value().ok().map(|v| AstNode::range(&v).start());
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
    let (config, deserialize_diagnostics) = deserialized.consume();

    if !deserialize_diagnostics.is_empty() {
        for diag in deserialize_diagnostics {
            let error = diag
                .with_file_path(block.file_path())
                .with_file_source_code(code);
            diagnostics.write_diagnostic(error);
        }
        if block.expect_diagnostic {
            return Ok(None);
        } else {
            diagnostics.print_all_diagnostics();
            bail!("Please fix the configuration errors above.");
        };
    }

    if config.is_none() {
        bail!(
            "Failed to deserialize configuration options for '{group}/{}' from the following code block due to unknown error.\n\n{code}",
            rule_metadata.name
        );
    }

    Ok(config)
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
                let test = CodeBlock::from_str(meta.as_ref())?;
                language = Some((test, String::new()));
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((test, block)) = language.take() {
                    if test.options != OptionsParsingMode::NoOptions {
                        last_options =
                            parse_rule_options(group, &rule_metadata, category, &test, &block)?;
                    } else {
                        if let Some(file_path) = test.explicit_file_path() {
                            test_runner
                                .file_system
                                .insert(file_path.to_string(), block.clone());
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
    test: CodeBlock,
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

    /// Runs all pending tests with the current file system.
    ///
    /// Resets state for the next section.
    pub fn run_pending_tests(&mut self) -> anyhow::Result<()> {
        let services_builder =
            AnalyzerServicesBuilder::from_files(mem::take(&mut self.file_system));

        for test in self.pending_tests.drain(..) {
            assert_lint(
                self.group,
                self.rule_name,
                &test.test,
                &test.block,
                test.options_snapshot,
                &services_builder,
            )?;
        }

        Ok(())
    }
}
