//! This module is in charge of checking if the documentation and tests cases inside the Analyzer rules are correct.
//!
//!
use anyhow::{bail, ensure};
use biome_analyze::options::JsxRuntime;
use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, FixKind, GroupCategory, Queryable, RegistryVisitor, Rule,
    RuleCategory, RuleFilter, RuleGroup, RuleMetadata,
};
use biome_console::{markup, Console};
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage, ModuleKind};
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use biome_service::settings::WorkspaceSettings;
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
            parse_documentation(
                group,
                meta.name,
                meta.docs,
                !matches!(meta.fix_kind, FixKind::None),
            )?;
        }
    }

    Ok(())
}

enum BlockType {
    Js(JsFileSource),
    Json,
    Css,
    Foreign(String),
}

struct CodeBlockTest {
    block_type: BlockType,
    expect_diagnostic: bool,
    ignore: bool,
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
            block_type: BlockType::Foreign(String::new()),
            expect_diagnostic: false,
            ignore: false,
        };

        for token in tokens {
            match token {
                // Determine the language, using the same list of extensions as `compute_source_type_from_path_or_extension`
                "cjs" => {
                    test.block_type = BlockType::Js(
                        JsFileSource::js_module().with_module_kind(ModuleKind::Script),
                    );
                }
                "js" | "mjs" | "jsx" => {
                    test.block_type = BlockType::Js(JsFileSource::jsx());
                }
                "ts" | "mts" | "cts" => {
                    test.block_type = BlockType::Js(JsFileSource::ts());
                }
                "tsx" => {
                    test.block_type = BlockType::Js(JsFileSource::tsx());
                }
                "svelte" => {
                    test.block_type = BlockType::Js(JsFileSource::svelte());
                }
                "astro" => {
                    test.block_type = BlockType::Js(JsFileSource::astro());
                }
                "vue" => {
                    test.block_type = BlockType::Js(JsFileSource::vue());
                }
                "json" => {
                    test.block_type = BlockType::Json;
                }
                "css" => {
                    test.block_type = BlockType::Css;
                }
                // Other attributes
                "expect_diagnostic" => {
                    test.expect_diagnostic = true;
                }
                "ignore" => {
                    test.ignore = true;
                }
                // A catch-all to regard unknown tokens as foreign languages,
                // and do not run tests on these code blocks.
                _ => {
                    test.block_type = BlockType::Foreign(token.into());
                    test.ignore = true;
                }
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
    has_fix_kind: bool,
) -> anyhow::Result<()> {
    let file = format!("{group}/{rule}.js");

    let mut diagnostic_count = 0;

    let mut all_diagnostics = vec![];

    let mut write_diagnostic = |code: &str, diag: biome_diagnostics::Error| {
        let category = diag.category().map_or("", |code| code.name());

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
            }

            ensure!(
                diagnostic_count == 0,
                "analysis returned multiple diagnostics, code snippet: \n\n{}",
                code
            );
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

            bail!(format!(
                "analysis returned an unexpected diagnostic, code `snippet:\n\n{:?}\n\n{}",
                category, code
            ));
        }

        diagnostic_count += 1;
        Ok(())
    };
    if test.ignore {
        return Ok(());
    }
    let mut rule_has_code_action = false;
    let mut settings = WorkspaceSettings::default();
    let key = settings.insert_project(PathBuf::new());
    settings.register_current_project(key);
    match &test.block_type {
        BlockType::Js(source_type) => {
            // Temporary support for astro, svelte and vue code blocks
            let (code, source_type) = match source_type.as_embedding_kind() {
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
                _ => (code, *source_type),
            };

            let parse = biome_js_parser::parse(code, source_type, JsParserOptions::default());

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag
                        .with_file_path(file.clone())
                        .with_file_source_code(code);
                    write_diagnostic(code, error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let mut options = AnalyzerOptions::default();
                options.configuration.jsx_runtime = Some(JsxRuntime::default());
                let (_, diagnostics) = biome_js_analyze::analyze(
                    &root,
                    filter,
                    &options,
                    source_type,
                    None,
                    |signal| {
                        if let Some(mut diag) = signal.diagnostic() {
                            let category = diag.category().expect("linter diagnostic has no code");
                            let severity = settings.get_current_settings().expect("project").get_severity_from_rule_code(category).expect(
                                "If you see this error, it means you need to run cargo codegen-configuration",
                            );

                            for action in signal.actions() {
                                if !action.is_suppression() {
                                    rule_has_code_action = true;
                                    diag = diag.add_code_suggestion(action.into());
                                }
                            }

                            let error = diag
                                .with_severity(severity)
                                .with_file_path(file.clone())
                                .with_file_source_code(code);
                            let res = write_diagnostic(code, error);

                            // Abort the analysis on error
                            if let Err(err) = res {
                                return ControlFlow::Break(err);
                            }
                        }

                        ControlFlow::Continue(())
                    },
                );

                // Result is Some(_) if analysis aborted with an error
                for diagnostic in diagnostics {
                    write_diagnostic(code, diagnostic)?;
                }
            }

            if test.expect_diagnostic && rule_has_code_action && !has_fix_kind {
                bail!("The rule '{}' emitted code actions via `action` function, but you didn't mark rule with `fix_kind`.", rule)
            }

            if test.expect_diagnostic {
                // Fail the test if the analysis didn't emit any diagnostic
                ensure!(
                    diagnostic_count == 1,
                    "analysis of {}/{} returned no diagnostics.\n code snippet:\n {}",
                    group,
                    rule,
                    code
                );
            }
        }
        BlockType::Json => {
            let parse = biome_json_parser::parse_json(code, JsonParserOptions::default());

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag
                        .with_file_path(file.clone())
                        .with_file_source_code(code);
                    write_diagnostic(code, error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = AnalyzerOptions::default();
                let (_, diagnostics) = biome_json_analyze::analyze(
                    &root,
                    filter,
                    &options,
                    |signal| {
                        if let Some(mut diag) = signal.diagnostic() {
                            let category = diag.category().expect("linter diagnostic has no code");
                            let severity = settings.get_current_settings().expect("project").get_severity_from_rule_code(category).expect(
                                "If you see this error, it means you need to run cargo codegen-configuration",
                            );

                            for action in signal.actions() {
                                if !action.is_suppression() {
                                    rule_has_code_action = true;
                                    diag = diag.add_code_suggestion(action.into());
                                }
                            }

                            let error = diag
                                .with_severity(severity)
                                .with_file_path(file.clone())
                                .with_file_source_code(code);
                            let res = write_diagnostic(code, error);

                            // Abort the analysis on error
                            if let Err(err) = res {
                                return ControlFlow::Break(err);
                            }
                        }

                        ControlFlow::Continue(())
                    },
                );

                // Result is Some(_) if analysis aborted with an error
                for diagnostic in diagnostics {
                    write_diagnostic(code, diagnostic)?;
                }

                if test.expect_diagnostic && rule_has_code_action && !has_fix_kind {
                    bail!("The rule '{}' emitted code actions via `action` function, but you didn't mark rule with `fix_kind`.", rule)
                }
            }
        }
        BlockType::Css => {
            let parse = biome_css_parser::parse_css(code, CssParserOptions::default());

            if parse.has_errors() {
                for diag in parse.into_diagnostics() {
                    let error = diag
                        .with_file_path(file.clone())
                        .with_file_source_code(code);
                    write_diagnostic(code, error)?;
                }
            } else {
                let root = parse.tree();

                let rule_filter = RuleFilter::Rule(group, rule);
                let filter = AnalysisFilter {
                    enabled_rules: Some(slice::from_ref(&rule_filter)),
                    ..AnalysisFilter::default()
                };

                let options = AnalyzerOptions::default();
                let (_, diagnostics) = biome_css_analyze::analyze(
                    &root,
                    filter,
                    &options,
                    |signal| {
                        if let Some(mut diag) = signal.diagnostic() {
                            let category = diag.category().expect("linter diagnostic has no code");
                            let severity = settings.get_current_settings().expect("project").get_severity_from_rule_code(category).expect(
                                "If you see this error, it means you need to run cargo codegen-configuration",
                            );

                            for action in signal.actions() {
                                if !action.is_suppression() {
                                    rule_has_code_action = true;
                                    diag = diag.add_code_suggestion(action.into());
                                }
                            }

                            let error = diag
                                .with_severity(severity)
                                .with_file_path(file.clone())
                                .with_file_source_code(code);
                            let res = write_diagnostic(code, error);

                            // Abort the analysis on error
                            if let Err(err) = res {
                                return ControlFlow::Break(err);
                            }
                        }

                        ControlFlow::Continue(())
                    },
                );

                // Result is Some(_) if analysis aborted with an error
                for diagnostic in diagnostics {
                    write_diagnostic(code, diagnostic)?;
                }

                if test.expect_diagnostic && rule_has_code_action && !has_fix_kind {
                    bail!("The rule '{}' emitted code actions via `action` function, but you didn't mark rule with `fix_kind`.", rule)
                }
            }
        }
        // Foreign code blocks should be already ignored by tests
        BlockType::Foreign(block) => {
            bail!("Unrecognised block type {}", &block)
        }
    }

    Ok(())
}

/// Parse the documentation fragment for a lint rule (in markdown) and generates
/// the content for the corresponding documentation page
fn parse_documentation(
    group: &'static str,
    rule: &'static str,
    docs: &'static str,
    has_fix_kind: bool,
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
                    assert_lint(group, rule, &test, &block, has_fix_kind)?;
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
