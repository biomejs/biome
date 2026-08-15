//! This module is in charge of checking if the documentation and tests cases inside the Analyzer rules are correct.
//!
//!
use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter, Write};
use std::mem;
use std::str::FromStr;

use anyhow::bail;
use biome_analyze::{
    GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleDomain, RuleGroup,
    RuleMetadata,
};
use biome_configuration::Configuration;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::Severity;
use biome_graphql_syntax::GraphqlLanguage;
use biome_html_syntax::HtmlLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_markdown_syntax::MarkdownLanguage;
use biome_ruledoc_utils::{
    AnalyzerServicesBuilder, CodeBlock, DiagnosticConsoleWriter, DiagnosticWriter,
    OptionsParsingMode, RuleCodeAnalyzer, parse_rule_options,
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};

#[derive(Debug)]
struct Errors {
    message: String,
}
impl Errors {
    const fn new(message: String) -> Self {
        Self { message }
    }
}
impl std::error::Error for Errors {}
impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { message } = self;
        f.write_str(message)
    }
}

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
            let group = R::Group::NAME;
            let rule_name = R::METADATA.name;
            let rule_severity = R::METADATA.severity;

            if TypeId::of::<R::Options>() == TypeId::of::<()>() {
                self.errors.push(Errors::new(format!(
                    "The rule '{rule_name}' uses `type Options = ()`. All lint rules must use a generated options struct (e.g., `RuleNameOptions`), even if empty. One should have been created for you if you ran the codegen when creating the rule. Create an empty options struct for this rule in biome_rule_options and update the rule to use it (e.g., `type Options = RuleNameOptions`)."
                )));
            }

            if let Some(issue_number) = R::METADATA.issue_number
                && group != "nursery"
            {
                self.errors.push(Errors::new(format!(
                    "The rule '{rule_name}' has an issue number set to '{issue_number}'. The presence of an issue number indicates that the rule is not yet completed. Rules that have an issue number must belong to the 'nursery' group. Change the group of the rule to 'nursery' or remove the issue number."
                )));
            }

            if matches!(group, "a11y" | "correctness" | "security")
                && rule_severity != Severity::Error
                && !matches!(
                    rule_name,
                    // TODO: remove these exceptions in Biome 3.0
                    "noNodejsModules"
                        | "noPrivateImports"
                        | "noUnusedFunctionParameters"
                        | "noUnusedImports"
                        | "noUnusedLabels"
                        | "noUnusedPrivateClassMembers"
                        | "noUnusedVariables"
                        | "useImportExtensions"
                        | "noNoninteractiveElementInteractions"
                        | "noGlobalDirnameFilename"
                        | "noProcessGlobal"
                        | "noReactPropAssignments"
                        | "noRestrictedElements"
                        | "noSecrets"
                        | "noSolidDestructuredProps"
                        | "useJsonImportAttributes"
                        | "useParseIntRadix"
                        | "useSingleJsDocAsterisk"
                )
            {
                self.errors.push(Errors::new(format!(
                    "The rule '{rule_name}' belongs to the group '{group}' and has a severity set to '{rule_severity}'. Rules that belong to the group {group} must have a severity set to 'error'. Set the severity to 'error' or change the group of the rule."
                )));
            } else if matches!(group, "complexity" | "style") && rule_severity == Severity::Error {
                self.errors.push(Errors::new(format!(
                    "The rule '{rule_name}' belongs to the group '{group}' and has a severity set to '{rule_severity}'. Rules that belong to the group '{group}' must not have a severity set to 'error'. Lower down the severity or change the group of the rule."
                )));
            } else if group == "performance"
                && rule_severity != Severity::Warning
                && !matches!(
                    rule_name,
                    // TODO: remove these exceptions in Biome 3.0
                    "noAwaitInLoops" | "useGoogleFontPreconnect" | "useSolidForComponent"
                )
            {
                self.errors.push(Errors::new(format!(
                    "The rule '{rule_name}' belongs to the group '{group}' and has a severity set to '{rule_severity}'. Rules that belong to the group '{group}' must have a severity set to 'warn'. Set the severity to 'warn' or change the group of the rule."
                )));
            } else if group == "suspicious"
                && rule_severity == Severity::Information
                && !matches!(
                    rule_name,
                    // TODO: remove these exceptions in Biome 3.0
                    "noAlert"
                        | "noBitwiseOperators"
                        | "noConstantBinaryExpressions"
                        | "noUnassignedVariables"
                        | "useStaticResponseMethods"
                        | "noQuickfixBiome"
                        | "noDuplicateFields"
                )
            {
                self.errors.push(Errors::new(format!(
                    "The rule '{rule_name}' belongs to the group '{group}' and has a severity set to '{rule_severity}'. Rules that belong to the group '{group}' must have a severity set to 'warn' or 'error'. Change the severity or change the group of the rule."
                )));
            } else if <R::Group as RuleGroup>::Category::CATEGORY == RuleCategory::Action
                && rule_severity != Severity::Information
            {
                self.errors.push(Errors::new(format!(
                    "The action '{rule_name}' has a severity set to '{rule_severity}'. Actions must have a severity set to 'info'. Set the severity of the rule to 'info'."
                )));
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

    impl RegistryVisitor<MarkdownLanguage> for LintRulesVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = MarkdownLanguage, Output: Clone>>
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
    biome_markdown_analyze::visit_registry(&mut visitor);

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

/// Parse and analyze the provided code block, and asserts that it emits
/// exactly zero or one diagnostic depending on the value of `expect_diagnostic`.
/// That diagnostic is then emitted as text into the `content` buffer
fn assert_lint(
    group: &'static str,
    rule: &'static str,
    rule_language: &'static str,
    test: &CodeBlock,
    code: &str,
    configuration: Option<Configuration>,
    services_builder: &mut AnalyzerServicesBuilder,
) -> anyhow::Result<()> {
    if test.ignore {
        return Ok(());
    }

    let mut diagnostics = DiagnosticConsoleWriter::default();
    RuleCodeAnalyzer {
        group,
        rule,
        rule_language,
        code_block: test,
        code,
        configuration,
        services_builder,
        writer: &mut diagnostics,
    }
    .analyze()?;

    if diagnostics.has_parse_error {
        // Fail if there is a parse error...
        diagnostics.print_all_diagnostics()?;
        bail!(
            "Analysis of '{group}/{rule}' on the following code block resulted in a parse error.\n\n{code}"
        );
    } else if test.expect_diagnostic {
        // ...or if the analysis does not return exactly one diagnostic...
        if diagnostics.all_diagnostics.len() != 1 {
            diagnostics.print_all_diagnostics()?;
            bail!(
                "Analysis of '{group}/{rule}' on the following code block returned {num_diagnostics} diagnostics, but a single diagnostic was expected.\n\n{code}",
                num_diagnostics = diagnostics.all_diagnostics.len()
            );
        }
    } else if test.expect_diff {
        // ...or there is no diff...
        if diagnostics.action_count == 0 {
            bail!(
                "Analysis of '{group}/{rule}' on the following code block returned no diff where one was expected.\n\n{code}",
            );
        }
    } else if !diagnostics.all_diagnostics.is_empty() {
        // ...or if the analysis returns a diagnostic when none are expected.
        diagnostics.print_all_diagnostics()?;
        bail!(
            "Analysis of '{group}/{rule}' on the following code block returned an unexpected diagnostic.\n\n{code}"
        );
    }

    Ok(())
}

/// Parse the documentation fragment for a lint rule (in markdown) and lint the code blocks.
fn parse_documentation(
    group: &'static str,
    rule_metadata: RuleMetadata,
    category: RuleCategory,
) -> anyhow::Result<()> {
    let parser = Parser::new(rule_metadata.docs);

    let mut diagnostics_writer = DiagnosticConsoleWriter::default();

    let mut test_runner = TestRunner::new(
        group,
        rule_metadata.name,
        rule_metadata.language,
        rule_metadata.domains.contains(&RuleDomain::Types),
    );

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
                        last_options = parse_rule_options(
                            group,
                            &rule_metadata,
                            category,
                            &test,
                            &block,
                            &mut diagnostics_writer,
                        )?;
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
                    write!(block, "{text}")?;
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
    rule_language: &'static str,
    enable_type_inference: bool,

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
    pub fn new(
        group: &'static str,
        rule_name: &'static str,
        rule_language: &'static str,
        enable_type_inference: bool,
    ) -> Self {
        Self {
            group,
            rule_name,
            rule_language,
            enable_type_inference,
            pending_tests: Vec::new(),
            file_system: HashMap::new(),
        }
    }

    /// Runs all pending tests with the current file system.
    ///
    /// Resets state for the next section.
    pub fn run_pending_tests(&mut self) -> anyhow::Result<()> {
        let mut services_builder = AnalyzerServicesBuilder::from_files(
            mem::take(&mut self.file_system),
            self.enable_type_inference,
        );

        for test in self.pending_tests.drain(..) {
            assert_lint(
                self.group,
                self.rule_name,
                self.rule_language,
                &test.test,
                &test.block,
                test.options_snapshot,
                &mut services_builder,
            )?;
        }

        Ok(())
    }
}
