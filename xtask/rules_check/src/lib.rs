//! This module is in charge of checking if the documentation and tests cases inside the Analyzer rules are correct.
//!
//!
use anyhow::{bail, ensure};
use biome_analyze::options::JsxRuntime;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, GroupCategory, Queryable,
    RegistryVisitor, Rule, RuleCategory, RuleFilter, RuleGroup, RuleMetadata,
};
use biome_console::{markup, Console};
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage};
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use biome_service::settings::WorkspaceSettings;
use biome_service::workspace::DocumentFileSource;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::ops::ControlFlow;
use std::path::PathBuf;
use std::slice;
use std::str::FromStr;

pub fn check_rules() -> anyhow::Result<()> {
    #[derive(Default)]
    struct LintRulesVisitor {
        groups: BTreeMap<&'static str, BTreeMap<&'static str, RuleMetadata>>,
    }

    impl RegistryVisitor<JsLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = JsLanguage>,
            <R::Query as Queryable>::Output: Clone,
        {
            self.groups
                .entry(<R::Group as RuleGroup>::NAME)
                .or_default()
                .insert(R::METADATA.name, R::METADATA);
        }
    }

    impl RegistryVisitor<JsonLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = JsonLanguage>,
            <R::Query as Queryable>::Output: Clone,
        {
            self.groups
                .entry(<R::Group as RuleGroup>::NAME)
                .or_default()
                .insert(R::METADATA.name, R::METADATA);
        }
    }

    impl RegistryVisitor<CssLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = CssLanguage>,
            <R::Query as Queryable>::Output: Clone,
        {
            self.groups
                .entry(<R::Group as RuleGroup>::NAME)
                .or_default()
                .insert(R::METADATA.name, R::METADATA);
        }
    }

    let mut visitor = LintRulesVisitor::default();
    biome_js_analyze::visit_registry(&mut visitor);
    biome_json_analyze::visit_registry(&mut visitor);
    biome_css_analyze::visit_registry(&mut visitor);

    let LintRulesVisitor { groups } = visitor;

    for (group, rules) in groups {
        for (_, meta) in rules {
            parse_documentation(group, meta.name, meta.docs)?;
        }
    }

    Ok(())
}
struct CodeBlockTest {
    tag: String,
    expect_diagnostic: bool,
    ignore: bool,
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
            .split(|c| c == ',' || c == ' ' || c == '\t')
            .map(str::trim)
            .filter(|token| !token.is_empty());

        let mut test = CodeBlockTest {
            tag: String::new(),
            expect_diagnostic: false,
            ignore: false,
        };

        for token in tokens {
            match token {
                // Other attributes
                "expect_diagnostic" => test.expect_diagnostic = true,
                "ignore" => test.ignore = true,
                // Regard as language tags, last one wins
                _ => test.tag = token.to_string(),
            }
        }

        Ok(test)
    }
}

/// Parse and analyze the provided code block, and asserts that it emits
/// exactly zero or one diagnostic depending on the value of `expect_diagnostic`.
/// That diagnostic is then emitted as text into the `content` buffer
fn assert_lint(
    group: &'static str,
    rule: &'static str,
    test: &CodeBlockTest,
    code: &str,
) -> anyhow::Result<()> {
    let file_path = format!("code-block.{}", test.tag);

    let mut diagnostic_count = 0;
    let mut all_diagnostics = vec![];
    let mut write_diagnostic = |code: &str, diag: biome_diagnostics::Error| {
        all_diagnostics.push(diag);
        // Fail the test if the analysis returns more diagnostics than expected
        if test.expect_diagnostic {
            // Print all diagnostics to help the user
            if all_diagnostics.len() > 1 {
                let mut console = biome_console::EnvConsole::default();
                for diag in all_diagnostics.iter() {
                    console.println(
                        biome_console::LogLevel::Error,
                        markup! {
                            {PrintDiagnostic::verbose(diag)}
                        },
                    );
                }
                bail!("Analysis of '{group}/{rule}' on the following code block returned multiple diagnostics.\n\n{code}");
            }
        } else {
            // Print all diagnostics to help the user
            let mut console = biome_console::EnvConsole::default();
            for diag in all_diagnostics.iter() {
                console.println(
                    biome_console::LogLevel::Error,
                    markup! {
                        {PrintDiagnostic::verbose(diag)}
                    },
                );
            }
            bail!("Analysis of '{group}/{rule}' on the following code block returned an unexpected diagnostic.\n\n{code}");
        }
        diagnostic_count += 1;
        Ok(())
    };

    if test.ignore {
        return Ok(());
    }

    let mut settings = WorkspaceSettings::default();
    let key = settings.insert_project(PathBuf::new());
    settings.register_current_project(key);
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
                    write_diagnostic(code, error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = AnalyzerOptions {
                    configuration: AnalyzerConfiguration {
                        jsx_runtime: Some(JsxRuntime::default()),
                        ..Default::default()
                    },
                    file_path: PathBuf::from(&file_path),
                };
                biome_js_analyze::analyze(&root, filter, &options, file_source, None, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        let category = diag.category().expect("linter diagnostic has no code");
                        let severity = settings.get_current_settings().expect("project").get_severity_from_rule_code(category).expect(
                                "If you see this error, it means you need to run cargo codegen-configuration",
                            );

                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag
                            .with_severity(severity)
                            .with_file_path(&file_path)
                            .with_file_source_code(code);
                        let res = write_diagnostic(code, error);

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
                    write_diagnostic(code, error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = AnalyzerOptions {
                    file_path: PathBuf::from(&file_path),
                    ..Default::default()
                };
                biome_json_analyze::analyze(&root, filter, &options, file_source, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        let category = diag.category().expect("linter diagnostic has no code");
                        let severity = settings.get_current_settings().expect("project").get_severity_from_rule_code(category).expect(
                                "If you see this error, it means you need to run cargo codegen-configuration",
                            );

                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag
                            .with_severity(severity)
                            .with_file_path(&file_path)
                            .with_file_source_code(code);
                        let res = write_diagnostic(code, error);

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
                    write_diagnostic(code, error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = AnalyzerOptions {
                    file_path: PathBuf::from(&file_path),
                    ..Default::default()
                };
                biome_css_analyze::analyze(&root, filter, &options, |signal| {
                    if let Some(mut diag) = signal.diagnostic() {
                        let category = diag.category().expect("linter diagnostic has no code");
                        let severity = settings.get_current_settings().expect("project").get_severity_from_rule_code(category).expect(
                                "If you see this error, it means you need to run cargo codegen-configuration",
                            );

                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diag = diag.add_code_suggestion(action.into());
                            }
                        }

                        let error = diag
                            .with_severity(severity)
                            .with_file_path(&file_path)
                            .with_file_source_code(code);
                        let res = write_diagnostic(code, error);

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
        // Unknown code blocks should be ignored by tests
        DocumentFileSource::Unknown | DocumentFileSource::Graphql(_) => {}
    }

    if test.expect_diagnostic {
        // Fail the test if the analysis didn't emit any diagnostic
        ensure!(
            diagnostic_count == 1,
            "Analysis of '{group}/{rule}' on the following code block returned no diagnostics.\n\n{code}",
        );
    }

    Ok(())
}

/// Parse the documentation fragment for a lint rule (in markdown) and lint the code blcoks.
fn parse_documentation(
    group: &'static str,
    rule: &'static str,
    docs: &'static str,
) -> anyhow::Result<()> {
    let parser = Parser::new(docs);

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
                    assert_lint(group, rule, &test, &block)?;
                }
            }
            Event::Text(text) => {
                if let Some((_, block)) = &mut language {
                    write!(block, "{text}")?;
                }
            }
            // We don't care other events
            _ => {}
        }
    }

    Ok(())
}
