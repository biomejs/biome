use crate::{AnalyzerServicesBuilder, CodeBlock, DiagnosticWriter};
use anyhow::Result;
use biome_analyze::{ActionFilter, AnalysisFilter, AnalyzerSignal, ControlFlow, RuleFilter};
use biome_configuration::Configuration;
use biome_css_analyze::CssAnalyzerServices;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::{Diagnostic, DiagnosticExt, advice::CodeSuggestionAdvice};
use biome_graphql_syntax::GraphqlLanguage;
use biome_html_parser::HtmlParserOptions;
use biome_html_syntax::HtmlLanguage;
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsLanguage;
use biome_json_analyze::JsonAnalyzeServices;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use biome_languages::{
    DocumentFileSource, HtmlFileSource,
    javascript::{JsEmbeddingKind, JsFileSource},
};
use biome_markdown_parser::MarkdownParserOptions;
use biome_markdown_syntax::MarkdownLanguage;
use biome_rowan::{Language, TextSize};
use biome_text_edit::TextEdit;
use biome_yaml_syntax::YamlLanguage;
use camino::Utf8PathBuf;
use std::slice;

/// Analyzes a documentation code block with a single rule enabled.
///
/// Diagnostics, parse errors, and code actions are sent to `writer`.
pub struct RuleCodeAnalyzer<'a> {
    pub group: &'static str,
    pub rule: &'static str,
    pub rule_language: &'static str,
    pub code_block: &'a CodeBlock,
    pub code: &'a str,
    pub configuration: Option<Configuration>,
    pub services_builder: &'a mut AnalyzerServicesBuilder,
    pub writer: &'a mut dyn DiagnosticWriter,
}

impl RuleCodeAnalyzer<'_> {
    /// Parses the code block and runs the selected rule.
    pub fn analyze(self) -> Result<()> {
        analyze_rule_code(self)
    }
}

/// Parses a documentation code block and runs the rule selected by `analyzer`.
pub fn analyze_rule_code(analyzer: RuleCodeAnalyzer) -> Result<()> {
    let RuleCodeAnalyzer {
        group,
        rule,
        rule_language,
        code_block,
        code,
        configuration,
        services_builder,
        writer,
    } = analyzer;

    if code_block.ignore {
        return Ok(());
    }

    let file_path = code_block.file_path();
    let rule_filter = RuleFilter::Rule(group, rule);
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };
    let html_file_source = if rule_language != "html" {
        if let Some(explicit_path) = code_block.explicit_file_path() {
            camino::Utf8Path::new(explicit_path)
                .extension()
                .and_then(|ext| HtmlFileSource::try_from_extension(ext).ok())
        } else {
            HtmlFileSource::try_from_extension(&code_block.tag).ok()
        }
    } else {
        None
    };

    if let Some(html_file_source) = html_file_source {
        let parse = biome_html_parser::parse_html(code, HtmlParserOptions::from(&html_file_source));

        if parse.has_errors() {
            for diagnostic in parse.into_diagnostics() {
                writer.write_parse_error(
                    diagnostic
                        .with_file_path(&file_path)
                        .with_file_source_code(code),
                )?;
            }
        } else {
            let settings = code_block.create_settings(configuration.clone())?;
            let snippets = services_builder.store_html_parse(
                Utf8PathBuf::from(&file_path),
                parse,
                html_file_source,
                &settings,
            );

            let mut has_parse_errors = false;
            for snippet in &snippets {
                for diagnostic in snippet.parse.diagnostics() {
                    has_parse_errors = true;
                    let span = diagnostic
                        .location()
                        .span
                        .map(|s| s + snippet.range.start());
                    writer.write_parse_error(
                        diagnostic
                            .clone()
                            .with_file_span(span)
                            .with_file_path(&file_path)
                            .with_file_source_code(code),
                    )?;
                }
            }

            if !has_parse_errors {
                for snippet in &snippets {
                    let root = snippet.parse.tree();
                    let options =
                        code_block.create_analyzer_options::<JsLanguage>(configuration.clone())?;
                    let services = services_builder.build_for_js_snippet(snippet);
                    let result = biome_js_analyze::analyze(
                        &root,
                        filter,
                        &options,
                        &[],
                        services,
                        |signal| {
                            process_signal(
                                signal,
                                code,
                                &file_path,
                                writer,
                                Some(snippet.range.start()),
                                Some(&code[snippet.range]),
                            )
                        },
                    );
                    propagate_break(result)?;
                }
            }
        }

        return Ok(());
    }

    let document_file_source = if rule_language == "html" {
        DocumentFileSource::Html(
            HtmlFileSource::try_from_extension(&code_block.tag)
                .unwrap_or_else(|_| HtmlFileSource::html()),
        )
    } else {
        code_block.document_file_source_from_path()
    };

    match document_file_source {
        DocumentFileSource::Js(file_source) => {
            let (analysis_code, file_source) = match file_source.as_embedding_kind() {
                JsEmbeddingKind::Astro { .. } => (
                    biome_service::file_handlers::AstroFileHandler::input(code),
                    JsFileSource::ts(),
                ),
                JsEmbeddingKind::Svelte { .. } => (
                    biome_service::file_handlers::SvelteFileHandler::input(code),
                    biome_service::file_handlers::SvelteFileHandler::file_source(code),
                ),
                JsEmbeddingKind::Vue { .. } => (
                    biome_service::file_handlers::VueFileHandler::input(code),
                    biome_service::file_handlers::VueFileHandler::file_source(code),
                ),
                _ => (code, file_source),
            };
            let parse =
                biome_js_parser::parse(analysis_code, file_source, JsParserOptions::default());

            if parse.has_errors() {
                for diagnostic in parse.into_diagnostics() {
                    writer.write_parse_error(
                        diagnostic
                            .with_file_path(&file_path)
                            .with_file_source_code(analysis_code),
                    )?;
                }
            } else {
                let root = parse.tree();
                let options = code_block.create_analyzer_options::<JsLanguage>(configuration)?;
                let services = services_builder.build_for_js_parse(
                    Utf8PathBuf::from(&file_path),
                    parse,
                    file_source,
                );
                let result =
                    biome_js_analyze::analyze(&root, filter, &options, &[], services, |signal| {
                        process_signal(signal, analysis_code, &file_path, writer, None, None)
                    });
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Json(file_source) => {
            let parse = biome_json_parser::parse_json(code, JsonParserOptions::from(&file_source));

            if parse.has_errors() {
                for diagnostic in parse.into_diagnostics() {
                    writer.write_parse_error(
                        diagnostic
                            .with_file_path(&file_path)
                            .with_file_source_code(code),
                    )?;
                }
            } else {
                let root = parse.tree();
                let options = code_block.create_analyzer_options::<JsonLanguage>(configuration)?;
                let services = JsonAnalyzeServices {
                    file_source,
                    configuration_provider: None,
                    project_layout: None,
                };
                let result =
                    biome_json_analyze::analyze(&root, filter, &options, services, &[], |signal| {
                        process_signal(signal, code, &file_path, writer, None, None)
                    });
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Css(file_source) => {
            let parse_options = CssParserOptions::from(&file_source);
            let parse = biome_css_parser::parse_css(code, file_source, parse_options);

            if parse.has_errors() {
                for diagnostic in parse.into_diagnostics() {
                    writer.write_parse_error(
                        diagnostic
                            .with_file_path(&file_path)
                            .with_file_source_code(code),
                    )?;
                }
            } else {
                let root = parse.tree();
                let options = code_block.create_analyzer_options::<CssLanguage>(configuration)?;
                let services = CssAnalyzerServices::default().with_file_source(file_source);
                let result =
                    biome_css_analyze::analyze(&root, filter, &options, services, &[], |signal| {
                        process_signal(signal, code, &file_path, writer, None, None)
                    });
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Graphql(_) => {
            let parse = biome_graphql_parser::parse_graphql(code);

            if parse.has_errors() {
                for diagnostic in parse.into_diagnostics() {
                    writer.write_parse_error(
                        diagnostic
                            .with_file_path(&file_path)
                            .with_file_source_code(code),
                    )?;
                }
            } else {
                let root = parse.tree();
                let options =
                    code_block.create_analyzer_options::<GraphqlLanguage>(configuration)?;
                let result = biome_graphql_analyze::analyze(&root, filter, &options, |signal| {
                    process_signal(signal, code, &file_path, writer, None, None)
                });
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Html(file_source) => {
            let parse = biome_html_parser::parse_html(code, HtmlParserOptions::from(&file_source));

            if parse.has_errors() {
                for diagnostic in parse.into_diagnostics() {
                    writer.write_parse_error(
                        diagnostic
                            .with_file_path(&file_path)
                            .with_file_source_code(code),
                    )?;
                }
            } else {
                let root = parse.tree();
                let services = services_builder.build_for_html_parse(
                    Utf8PathBuf::from(&file_path),
                    parse,
                    file_source,
                );
                let options = code_block.create_analyzer_options::<HtmlLanguage>(configuration)?;
                let result = biome_html_analyze::analyze(
                    &root,
                    filter,
                    &options,
                    file_source,
                    services,
                    None,
                    |signal| process_signal(signal, code, &file_path, writer, None, None),
                );
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Markdown(file_source) => {
            let parse =
                biome_markdown_parser::parse_markdown(code, MarkdownParserOptions::default());

            if parse.has_errors() {
                for diagnostic in parse.into_diagnostics() {
                    writer.write_parse_error(
                        diagnostic
                            .with_file_path(&file_path)
                            .with_file_source_code(code),
                    )?;
                }
            } else {
                let root = parse.tree();
                let options =
                    code_block.create_analyzer_options::<MarkdownLanguage>(configuration)?;
                let result = biome_markdown_analyze::analyze(
                    &root,
                    filter,
                    &options,
                    file_source,
                    |signal| process_signal(signal, code, &file_path, writer, None, None),
                );
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Grit(_) => todo!("Grit analysis is not yet supported"),
        DocumentFileSource::Yaml(_) => {
            let parse = biome_yaml_parser::parse_yaml(code);

            if parse.has_errors() {
                for diagnostic in parse.into_diagnostics() {
                    writer.write_parse_error(
                        diagnostic
                            .with_file_path(&file_path)
                            .with_file_source_code(code),
                    )?;
                }
            } else {
                let root = parse.tree();
                let options = code_block.create_analyzer_options::<YamlLanguage>(configuration)?;
                let result = biome_yaml_analyze::analyze(&root, filter, &options, |signal| {
                    process_signal(signal, code, &file_path, writer, None, None)
                });
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Unknown | DocumentFileSource::Ignore => {}
    }

    Ok(())
}

fn process_signal<L: Language>(
    signal: &dyn AnalyzerSignal<L>,
    source: &str,
    file_path: &str,
    writer: &mut dyn DiagnosticWriter,
    snippet_offset: Option<TextSize>,
    snippet_text: Option<&str>,
) -> ControlFlow<anyhow::Error> {
    let actions = signal.actions(ActionFilter::rule_fix()).collect::<Vec<_>>();

    let mut full_edits = Vec::with_capacity(actions.len());
    for action in &actions {
        let edit = action
            .text_edit
            .clone()
            .or_else(|| action.mutation.clone().to_text_range_and_edit())
            .map(|(_, e)| e);
        if let Some(edit) = edit {
            if let (Some(offset), Some(snippet_text)) = (snippet_offset, snippet_text) {
                let new_snippet = edit.new_string(snippet_text);
                let start = u32::from(offset) as usize;
                let end = start + snippet_text.len();
                let mut new_source = source.to_string();
                new_source.replace_range(start..end, &new_snippet);
                let full_edit = TextEdit::from_unicode_words(source, &new_source);
                full_edits.push(Some(full_edit));
            } else {
                full_edits.push(Some(edit));
            }
        } else {
            full_edits.push(None);
        }
    }

    if let Some(mut diagnostic) = signal.diagnostic() {
        if let Some(offset) = snippet_offset {
            diagnostic.add_diagnostic_offset(offset);
        }
        for (action, full_edit) in actions.iter().zip(&full_edits) {
            if let Some(full_edit) = full_edit {
                diagnostic = diagnostic.add_code_suggestion(CodeSuggestionAdvice {
                    applicability: action.applicability,
                    msg: action.message.clone(),
                    suggestion: full_edit.clone(),
                });
            } else {
                diagnostic = diagnostic.add_code_suggestion(action.clone().into());
            }
        }
        if let Err(error) = writer.write_diagnostic(
            diagnostic
                .with_file_path(file_path)
                .with_file_source_code(source),
        ) {
            return ControlFlow::Break(error);
        }
    }

    for edit in full_edits.into_iter().flatten() {
        if let Err(error) = writer.write_action(source, file_path, edit) {
            return ControlFlow::Break(error);
        }
    }

    ControlFlow::Continue(())
}

fn propagate_break<T>((result, _): (Option<anyhow::Error>, Vec<T>)) -> Result<()> {
    if let Some(error) = result {
        Err(error)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiagnosticConsoleWriter;
    use std::{collections::HashMap, str::FromStr};

    #[test]
    fn resolves_relative_context_files_from_absolute_code_blocks() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(
            HashMap::from([("foo.js".to_string(), "export const foo = 1;".to_string())]),
            false,
        );
        let code_block =
            CodeBlock::from_str("js expect_diagnostic file=bar.js").expect("valid code block");
        let code = r#"import { missing } from "./foo.js";"#;
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "correctness",
            rule: "noUnresolvedImports",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        let diagnostic = writer
            .all_diagnostics
            .pop()
            .expect("missing export diagnostic");
        let diagnostic = biome_test_utils::diagnostic_to_string("/bar.js", code, diagnostic);
        assert!(diagnostic.contains("has no export named missing"));
        assert!(!diagnostic.contains("module not found"));
    }

    #[test]
    fn analyzes_astro_template_expression_for_js_rule() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "{show && <img src=\"avatar.png\" />}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        let diagnostic = writer.all_diagnostics.pop().expect("useAltText diagnostic");
        let diagnostic =
            biome_test_utils::diagnostic_to_string("code-block.astro", code, diagnostic);
        assert!(diagnostic.contains("Provide a text alternative"));
    }

    #[test]
    fn analyzes_valid_astro_template_expression_for_js_rule() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro").expect("valid code block");
        let code = "{show && <img src=\"avatar.png\" alt=\"avatar\" />}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert!(writer.all_diagnostics.is_empty());
    }

    #[test]
    fn analyzes_astro_jsx_nested_expression() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "{items.map(item => <img src={item} />)}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic =
            biome_test_utils::diagnostic_to_string("code-block.astro", code, diagnostic);
        assert!(diagnostic.contains("Provide a text alternative"));
    }

    #[test]
    fn analyzes_multiple_embedded_expressions() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code =
            "---\nconst raw = 'avatar.png';\n---\n<h1>{raw}</h1>\n{show && <img src={raw} />}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic =
            biome_test_utils::diagnostic_to_string("code-block.astro", code, diagnostic);
        assert!(diagnostic.contains("Provide a text alternative"));
    }

    #[test]
    fn analyzes_astro_markup() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "<div class=\"wrapper\">\n  {<img src=\"avatar.png\" />}\n</div>";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic =
            biome_test_utils::diagnostic_to_string("code-block.astro", code, diagnostic);
        assert!(diagnostic.contains("Provide a text alternative"));
    }

    #[test]
    fn analyzes_plain_jsx() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("jsx expect_diagnostic").expect("valid code block");
        let code = "var element = <span />;";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic = biome_test_utils::diagnostic_to_string("code-block.jsx", code, diagnostic);
        assert!(diagnostic.contains("Use let or const instead of var"));
    }

    #[test]
    fn analyzes_tsx() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("tsx expect_diagnostic").expect("valid code block");
        let code = "var element: JSX.Element = <span />;";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic = biome_test_utils::diagnostic_to_string("code-block.tsx", code, diagnostic);
        assert!(diagnostic.contains("Use let or const instead of var"));
    }

    #[test]
    fn precedence_tag_js_explicit_file_astro() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("js file=component.astro expect_diagnostic")
            .expect("valid code block");
        let code = "{show && <img src=\"avatar.png\" />}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic =
            biome_test_utils::diagnostic_to_string("component.astro", code, diagnostic);
        assert!(diagnostic.contains("Provide a text alternative"));
    }

    #[test]
    fn precedence_tag_astro_explicit_file_js() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro file=component.js expect_diagnostic")
            .expect("valid code block");
        let code = "var x = 1;";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic = biome_test_utils::diagnostic_to_string("component.js", code, diagnostic);
        assert!(diagnostic.contains("Use let or const instead of var"));
    }

    #[test]
    fn precedence_regular_astro_tag_without_explicit_file() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "{show && <img src=\"avatar.png\" />}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
    }

    #[test]
    fn precedence_regular_jsx_tag_without_explicit_file() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("jsx expect_diagnostic").expect("valid code block");
        let code = "var x = 1;";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
    }

    #[test]
    fn analyzes_code_block_options() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block =
            CodeBlock::from_str("astro use_options expect_diagnostic").expect("valid code block");
        let code = "{show && <img src=\"avatar.png\" />}";
        let mut writer = DiagnosticConsoleWriter::default();
        let config = biome_configuration::Configuration::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: Some(config),
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
    }

    #[test]
    fn reports_invalid_syntax_cases() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro").expect("valid code block");
        let code = "{for (;;;}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert!(writer.has_parse_error);
    }

    #[test]
    fn verifies_diagnostic_ranges_and_offsets() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "{show && <img src=\"avatar.png\" />}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        let diagnostic = writer
            .all_diagnostics
            .pop()
            .expect("diagnostic should be emitted");
        let span = diagnostic.location().span.expect("span should be present");
        let highlighted = &code[span];
        assert_eq!(highlighted, "<img src=\"avatar.png\" />");
        let printed = biome_test_utils::diagnostic_to_string("code-block.astro", code, diagnostic);
        assert!(printed.contains("Provide a text alternative"));
    }

    #[test]
    fn analyzes_plain_js() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("js expect_diagnostic").expect("valid code block");
        let code = "var x = 1;";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic = biome_test_utils::diagnostic_to_string("code-block.js", code, diagnostic);
        assert!(diagnostic.contains("Use let or const instead of var"));
    }

    #[test]
    fn analyzes_another_js_rule_on_astro_frontmatter() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "---\nvar greeting = 'hello';\n---\n<h1>{greeting}</h1>";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic =
            biome_test_utils::diagnostic_to_string("code-block.astro", code, diagnostic);
        assert!(diagnostic.contains("Use let or const instead of var"));
    }

    #[test]
    fn analyzes_another_jsx_rule_on_astro_template() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "{<img src=\"avatar.png\" />}";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "a11y",
            rule: "useAltText",
            rule_language: "jsx",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let diagnostic = writer.all_diagnostics.pop().unwrap();
        let diagnostic =
            biome_test_utils::diagnostic_to_string("code-block.astro", code, diagnostic);
        assert!(diagnostic.contains("Provide a text alternative"));
    }

    #[test]
    fn verifies_autofix_ranges_on_embedded_code() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "---\nvar x = 1;\n---\n";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        assert_eq!(writer.action_count, 1);

        let diagnostic = &writer.all_diagnostics[0];
        let span = diagnostic.location().span.expect("span should be present");
        assert_eq!(&code[span], "var x = 1");

        let action_edit = writer
            .all_actions
            .pop()
            .expect("action edit should be present");
        let new_code = action_edit.new_string(code);
        assert_eq!(new_code, "---\nconst x = 1;\n---\n");
    }

    #[test]
    fn recognizes_template_references_in_astro_embedded_code() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro").expect("valid code block");
        let code = "---\nconst greeting = 'hello';\n---\n<h1>{greeting}</h1>";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "correctness",
            rule: "noUnusedVariables",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 0);
    }

    #[test]
    fn reports_actual_unused_variables_in_astro_embedded_code() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro expect_diagnostic").expect("valid code block");
        let code = "---\nconst greeting = 'hello';\nconst unused = 1;\n---\n<h1>{greeting}</h1>";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "correctness",
            rule: "noUnusedVariables",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert_eq!(writer.all_diagnostics.len(), 1);
        let span = writer.all_diagnostics[0].location().span.unwrap();
        assert_eq!(&code[span], "unused");
    }

    #[test]
    fn parses_astro_script_as_typescript() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("astro").expect("valid code block");
        let code = "<script>\nconst n: number = 1;\n</script>";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert!(!writer.has_parse_error);
        assert_eq!(writer.all_diagnostics.len(), 0);
    }

    #[test]
    fn parses_vue_and_svelte_tsx_scripts() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("vue").expect("valid code block");
        let code = "<script lang=\"tsx\">\nconst elem: JSX.Element = <div />;\n</script>";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert!(!writer.has_parse_error);
        assert_eq!(writer.all_diagnostics.len(), 0);
    }

    #[test]
    fn analyzes_js_rule_with_the_vue_model_of_the_file() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("vue expect_diagnostic").expect("valid code block");
        let code = "<script setup>\nconst emit = defineEmits([\"change\"]);\n</script>\n\n<template><button @click=\"emit('close')\"></button></template>";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "nursery",
            rule: "noVueUnusedEmitDeclarations",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert!(!writer.has_parse_error);
        assert_eq!(writer.all_diagnostics.len(), 1);
        let span = writer.all_diagnostics[0].location().span.unwrap();
        assert_eq!(&code[span], "\"change\"");
    }

    #[test]
    fn analyzes_each_code_block_at_the_same_path_on_its_own() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let analyze = |services_builder: &mut AnalyzerServicesBuilder, code: &str| {
            let code_block = CodeBlock::from_str("vue").expect("valid code block");
            let mut writer = DiagnosticConsoleWriter::default();
            RuleCodeAnalyzer {
                group: "correctness",
                rule: "noUnusedVariables",
                rule_language: "js",
                code_block: &code_block,
                code,
                configuration: None,
                services_builder,
                writer: &mut writer,
            }
            .analyze()
            .unwrap();
            writer.all_diagnostics.len()
        };

        let used =
            "<script setup>\nconst count = 1;\n</script>\n\n<template>{{ count }}</template>";
        let unused =
            "<script setup>\nconst count = 1;\n</script>\n\n<template><div></div></template>";
        assert_eq!(analyze(&mut services_builder, used), 0);
        assert_eq!(analyze(&mut services_builder, unused), 1);
        assert_eq!(analyze(&mut services_builder, used), 0);
    }

    #[test]
    fn parses_embedded_scripts_with_code_block_options() {
        let code = "<script lang=\"ts\">\nclass A { m(@dec x) {} }\n</script>";
        let parse_errors = |tag: &str, configuration: Option<Configuration>| {
            let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
            let code_block = CodeBlock::from_str(tag).expect("valid code block");
            let mut writer = DiagnosticConsoleWriter::default();
            RuleCodeAnalyzer {
                group: "suspicious",
                rule: "noVar",
                rule_language: "js",
                code_block: &code_block,
                code,
                configuration,
                services_builder: &mut services_builder,
                writer: &mut writer,
            }
            .analyze()
            .unwrap();
            writer
                .all_diagnostics
                .iter()
                .map(|diagnostic| &code[diagnostic.location().span.unwrap()])
                .collect::<Vec<_>>()
        };
        let configuration = Configuration {
            javascript: Some(biome_configuration::JsConfiguration {
                parser: Some(biome_configuration::javascript::JsParserConfiguration {
                    unsafe_parameter_decorators_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(parse_errors("vue", None), ["@dec"]);
        assert!(parse_errors("vue use_options", Some(configuration)).is_empty());
    }

    #[test]
    fn ignores_json_script_blocks() {
        let mut services_builder = AnalyzerServicesBuilder::from_files(HashMap::new(), false);
        let code_block = CodeBlock::from_str("html").expect("valid code block");
        let code = "<script type=\"application/json\">\n{\n  \"key\": \"value\"\n}\n</script>";
        let mut writer = DiagnosticConsoleWriter::default();

        RuleCodeAnalyzer {
            group: "suspicious",
            rule: "noVar",
            rule_language: "js",
            code_block: &code_block,
            code,
            configuration: None,
            services_builder: &mut services_builder,
            writer: &mut writer,
        }
        .analyze()
        .unwrap();

        assert!(!writer.has_parse_error);
        assert_eq!(writer.all_diagnostics.len(), 0);
    }
}
