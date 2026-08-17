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
use biome_markdown_syntax::MarkdownLanguage;
use biome_rowan::Language;
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
                let semantic_model = biome_css_semantic::semantic_model(&root);
                let services = CssAnalyzerServices::default()
                    .with_file_source(file_source)
                    .with_semantic_model(&semantic_model);
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
                let options = code_block.create_analyzer_options::<HtmlLanguage>(configuration)?;
                let result = biome_html_analyze::analyze(
                    &root,
                    filter,
                    &options,
                    file_source,
                    biome_html_analyze::HtmlAnalyzerServices::default(),
                    |signal| process_signal(signal, code, &file_path, writer),
                );
                propagate_break(result)?;
            }
        }
        DocumentFileSource::Markdown(_) => {
            let parse = biome_markdown_parser::parse_markdown(code);

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
}
