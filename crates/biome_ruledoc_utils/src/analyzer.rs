use crate::{AnalyzerServicesBuilder, CodeBlock, DiagnosticWriter};
use anyhow::Result;
use biome_analyze::{ActionFilter, AnalysisFilter, AnalyzerSignal, ControlFlow, RuleFilter};
use biome_configuration::Configuration;
use biome_css_analyze::CssAnalyzerServices;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::DiagnosticExt;
use biome_graphql_syntax::GraphqlLanguage;
use biome_html_parser::HtmlParserOptions;
use biome_html_syntax::{
    AnyAstroDirective, AstroEmbeddedContent, HtmlAttribute, HtmlElement, HtmlLanguage, HtmlRoot,
    HtmlTextExpression,
};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsLanguage;
use biome_json_analyze::JsonAnalyzeServices;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use biome_languages::{
    DocumentFileSource, HtmlFileSource,
    javascript::{JsEmbeddingKind, JsFileSource, SvelteEmbeddingKind, SvelteFileKind},
};
use biome_markdown_parser::MarkdownParserOptions;
use biome_markdown_syntax::MarkdownLanguage;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstNodeList, Language, NodeCache, TextSize};
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
            let mut node_cache = NodeCache::default();
            let html_root: HtmlRoot = parse.tree();
            let mut parser_options = JsParserOptions::default();
            if let Some(config) = configuration.as_ref()
                && let Some(js) = &config.javascript
                && let Some(parser) = &js.parser
                && parser
                    .unsafe_parameter_decorators_enabled
                    .is_some_and(|b| b.into())
            {
                parser_options = parser_options.with_parse_class_parameter_decorators();
            }

            let snippets = extract_html_embedded_js(code, &html_root, &html_file_source);

            for snippet in snippets {
                let parse = biome_js_parser::parse_js_with_offset_and_cache(
                    snippet.text,
                    snippet.offset,
                    snippet.file_source,
                    parser_options,
                    &mut node_cache,
                );
                let any_parse: AnyParse = parse.into();

                if any_parse.has_errors() {
                    for diagnostic in any_parse.into_diagnostics() {
                        writer.write_parse_error(
                            diagnostic
                                .with_file_path(&file_path)
                                .with_file_source_code(code),
                        )?;
                    }
                } else {
                    let root = any_parse.tree();
                    let options =
                        code_block.create_analyzer_options::<JsLanguage>(configuration.clone())?;
                    let services = services_builder.build_for_js_any_parse(
                        Utf8PathBuf::from(&file_path),
                        any_parse,
                        snippet.file_source,
                    );
                    let result = biome_js_analyze::analyze(
                        &root,
                        filter,
                        &options,
                        &[],
                        services,
                        |signal| process_signal(signal, code, &file_path, writer),
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
                        process_signal(signal, analysis_code, &file_path, writer)
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
                        process_signal(signal, code, &file_path, writer)
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
                        process_signal(signal, code, &file_path, writer)
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
                    process_signal(signal, code, &file_path, writer)
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
                    |signal| process_signal(signal, code, &file_path, writer),
                );
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Markdown(_) => {
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
                let result = biome_markdown_analyze::analyze(&root, filter, &options, |signal| {
                    process_signal(signal, code, &file_path, writer)
                });
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Grit(_) => todo!("Grit analysis is not yet supported"),
        DocumentFileSource::Yaml(_) => todo!("Yaml analysis is not yet supported"),
        DocumentFileSource::Unknown | DocumentFileSource::Ignore => {}
    }

    Ok(())
}

fn process_signal<L: Language>(
    signal: &dyn AnalyzerSignal<L>,
    source: &str,
    file_path: &str,
    writer: &mut dyn DiagnosticWriter,
) -> ControlFlow<anyhow::Error> {
    let actions = signal.actions(ActionFilter::rule_fix()).collect::<Vec<_>>();

    if let Some(mut diagnostic) = signal.diagnostic() {
        for action in &actions {
            diagnostic = diagnostic.add_code_suggestion(action.clone().into());
        }
        if let Err(error) = writer.write_diagnostic(
            diagnostic
                .with_file_path(file_path)
                .with_file_source_code(source),
        ) {
            return ControlFlow::Break(error);
        }
    }

    for action in actions {
        let Some((_, edit)) = action
            .text_edit
            .or_else(|| action.mutation.to_text_range_and_edit())
        else {
            continue;
        };
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

struct EmbeddedJsSnippet<'a> {
    text: &'a str,
    offset: TextSize,
    file_source: JsFileSource,
}

fn extract_html_embedded_js<'a>(
    code: &'a str,
    html_root: &HtmlRoot,
    host_file_source: &HtmlFileSource,
) -> Vec<EmbeddedJsSnippet<'a>> {
    let mut snippets = Vec::new();

    for node in html_root.syntax().descendants() {
        if let Some(frontmatter) = AstroEmbeddedContent::cast_ref(&node) {
            if let Some(token) = frontmatter.content_token() {
                let range = token.text_range();
                snippets.push(EmbeddedJsSnippet {
                    text: &code[range],
                    offset: range.start(),
                    file_source: JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Astro {
                        frontmatter: true,
                        is_class_attribute: false,
                    }),
                });
            }
        } else if let Some(element) = HtmlElement::cast_ref(&node) {
            if !element.has_astro_is_raw()
                && element
                    .tag_name()
                    .is_some_and(|name| name.text().eq_ignore_ascii_case("script"))
            {
                let is_ts = element
                    .opening_element()
                    .ok()
                    .into_iter()
                    .flat_map(|opening| opening.attributes())
                    .any(|attr| {
                        attr.as_html_attribute().is_some_and(|a| {
                            a.name()
                                .ok()
                                .and_then(|n| n.value_token().ok())
                                .is_some_and(|t| t.text_trimmed() == "lang")
                                && a.initializer()
                                    .and_then(|i| i.value().ok())
                                    .is_some_and(|v| {
                                        v.as_html_string().is_some_and(|s| {
                                            s.inner_string_text().is_ok_and(|t| t.text() == "ts")
                                        })
                                    })
                        })
                    });
                let base_source = if is_ts {
                    JsFileSource::ts()
                } else {
                    JsFileSource::js_module()
                };
                let file_source = if host_file_source.is_svelte() {
                    base_source.with_embedding_kind(JsEmbeddingKind::Svelte {
                        file_kind: SvelteFileKind::Component,
                        embedding_kind: SvelteEmbeddingKind::Source,
                    })
                } else if host_file_source.is_vue() {
                    let is_setup = element
                        .opening_element()
                        .ok()
                        .into_iter()
                        .flat_map(|opening| opening.attributes())
                        .any(|attr| {
                            attr.as_html_attribute()
                                .and_then(|a| a.name().ok())
                                .and_then(|n| n.value_token().ok())
                                .is_some_and(|t| t.text_trimmed() == "setup")
                        });
                    base_source.with_embedding_kind(JsEmbeddingKind::Vue {
                        setup: is_setup,
                        is_source: true,
                        event_handler: false,
                        allow_statements: true,
                    })
                } else {
                    base_source
                };

                let content_child = element.children().iter().find_map(|child| {
                    let child = child.as_any_html_content()?;
                    child.as_html_embedded_content().cloned()
                });
                if let Some(content_child) = content_child
                    && let Ok(token) = content_child.value_token()
                {
                    let range = token.text_range();
                    snippets.push(EmbeddedJsSnippet {
                        text: &code[range],
                        offset: range.start(),
                        file_source,
                    });
                }
            }
        } else if let Some(expr) = HtmlTextExpression::cast_ref(&node)
            && let Ok(token) = expr.html_literal_token()
        {
            let is_class_attribute = expr.syntax().ancestors().any(|ancestor| {
                if let Some(attr) = HtmlAttribute::cast_ref(&ancestor) {
                    return attr
                        .name()
                        .ok()
                        .and_then(|n| n.value_token().ok())
                        .is_some_and(|t| t.text_trimmed() == "class");
                }
                if let Some(directive) = AnyAstroDirective::cast_ref(&ancestor) {
                    return directive.as_astro_class_directive().is_some();
                }
                false
            });

            let file_source = if host_file_source.is_astro() {
                JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Astro {
                    frontmatter: false,
                    is_class_attribute,
                })
            } else if host_file_source.is_svelte() {
                JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Svelte {
                    file_kind: SvelteFileKind::Component,
                    embedding_kind: SvelteEmbeddingKind::Expression,
                })
            } else if host_file_source.is_vue() {
                JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Vue {
                    setup: false,
                    is_source: false,
                    event_handler: false,
                    allow_statements: false,
                })
            } else {
                JsFileSource::tsx()
            };

            let range = token.text_range();
            snippets.push(EmbeddedJsSnippet {
                text: &code[range],
                offset: range.start(),
                file_source,
            });
        }
    }

    snippets
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
        assert!(highlighted.contains("<img"));
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
    }
}
