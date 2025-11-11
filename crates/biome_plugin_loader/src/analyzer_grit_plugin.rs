use crate::{AnalyzerPlugin, PluginDiagnostic};
use biome_analyze::{PluginTargetLanguage, RuleDiagnostic};
use biome_console::markup;
use biome_css_syntax::{CssRoot, CssSyntaxNode};
use biome_diagnostics::{Severity, category};
use biome_fs::FileSystem;
use biome_grit_patterns::{
    BuiltInFunction, CompilePatternOptions, GritBinding, GritExecContext, GritPattern, GritQuery,
    GritQueryContext, GritQueryState, GritResolvedPattern, GritTargetFile, GritTargetLanguage,
    compile_pattern_with_options,
};
use biome_js_syntax::{AnyJsRoot, JsSyntaxNode};
use biome_parser::{AnyParse, NodeParse};
use biome_rowan::{AnySyntaxNode, AstNode, RawSyntaxKind, SyntaxKind, TextRange};
use camino::{Utf8Path, Utf8PathBuf};
use grit_pattern_matcher::{binding::Binding, pattern::ResolvedPattern};
use grit_util::{AnalysisLogs, error::GritPatternError};
use std::{borrow::Cow, fmt::Debug, str::FromStr, sync::Arc};

/// Definition of an analyzer plugin.
#[derive(Debug)]
pub struct AnalyzerGritPlugin {
    grit_query: GritQuery,
}

impl AnalyzerGritPlugin {
    pub fn load(fs: &dyn FileSystem, path: &Utf8Path) -> Result<Self, PluginDiagnostic> {
        let source = fs.read_file_from_path(path)?;
        let options = CompilePatternOptions::default()
            .with_extra_built_ins(vec![
                BuiltInFunction::new(
                    "register_diagnostic",
                    &["span", "message", "severity"],
                    Box::new(register_diagnostic),
                )
                .as_predicate(),
            ])
            .with_path(path);
        let grit_query = compile_pattern_with_options(&source, options)?;

        Ok(Self { grit_query })
    }
}

impl AnalyzerPlugin for AnalyzerGritPlugin {
    fn language(&self) -> PluginTargetLanguage {
        match &self.grit_query.language {
            GritTargetLanguage::JsTargetLanguage(_) => PluginTargetLanguage::JavaScript,
            GritTargetLanguage::CssTargetLanguage(_) => PluginTargetLanguage::Css,
        }
    }

    fn query(&self) -> Vec<RawSyntaxKind> {
        match self.language() {
            PluginTargetLanguage::JavaScript => AnyJsRoot::KIND_SET
                .iter()
                .map(|kind| kind.to_raw())
                .collect(),
            PluginTargetLanguage::Css => {
                CssRoot::KIND_SET.iter().map(|kind| kind.to_raw()).collect()
            }
        }
    }

    fn evaluate(&self, node: AnySyntaxNode, path: Arc<Utf8PathBuf>) -> Vec<RuleDiagnostic> {
        let name: &str = self.grit_query.name.as_deref().unwrap_or("anonymous");

        let root = match self.language() {
            PluginTargetLanguage::JavaScript => node
                .downcast_ref::<JsSyntaxNode>()
                .and_then(|node| node.as_send()),
            PluginTargetLanguage::Css => node
                .downcast_ref::<CssSyntaxNode>()
                .and_then(|node| node.as_send()),
        };

        let parse = AnyParse::Node(NodeParse::new(root.unwrap(), vec![]));
        let file = GritTargetFile { parse, path };

        match self.grit_query.execute(file) {
            Ok(result) => {
                let mut diagnostics: Vec<_> = result
                    .logs
                    .iter()
                    .map(|log| {
                        RuleDiagnostic::new(
                        category!("plugin"),
                        log.range.map(from_grit_range),
                        markup!(<Emphasis>{name}</Emphasis>" logged: "<Info>{log.message}</Info>),
                    )
                    .verbose()
                    })
                    .chain(result.diagnostics)
                    .map(|diagnostics| diagnostics.subcategory(name.to_string()))
                    .collect();

                if diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.span().is_none())
                {
                    diagnostics.push(RuleDiagnostic::new(
                        category!("plugin"),
                        None::<TextRange>,
                        markup!(
                            "Plugin "<Emphasis>{name}</Emphasis>" reported one or more diagnostics, "
                            "but it didn't specify a valid "<Emphasis>"span"</Emphasis>". "
                            "Diagnostics have been shown without context."
                        ),
                    ));
                }

                diagnostics
            }
            Err(error) => vec![RuleDiagnostic::new(
                category!("plugin"),
                None::<TextRange>,
                markup!(<Emphasis>{name}</Emphasis>" errored: "<Error>{error.to_string()}</Error>),
            )],
        }
    }
}

fn from_grit_range(range: grit_util::Range) -> TextRange {
    TextRange::new(range.start_byte.into(), range.end_byte.into())
}

fn register_diagnostic<'a>(
    args: &'a [Option<GritPattern<GritQueryContext>>],
    context: &'a GritExecContext,
    state: &mut GritQueryState<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>, GritPatternError> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;

    let (span_node, message, severity) = match args.as_slice() {
        [Some(span), Some(message), severity] => (span, message, severity),
        _ => {
            return Err(GritPatternError::new(
                "register_diagnostic() takes 2 required arguments: span and message, and an optional severity",
            ));
        }
    };

    let span = span_node
        .get_last_binding()
        .and_then(GritBinding::as_node)
        .map(|node| node.text_trimmed_range());

    let message = match message {
        GritResolvedPattern::Constant(constant) => Some(constant.to_string().into()),
        GritResolvedPattern::Snippets(snippets) => snippets
            .iter()
            .try_fold(Cow::Borrowed(""), |text, snippet| {
                let snippet_text = snippet.text(&state.files, &context.lang);
                if text.is_empty() {
                    snippet_text
                } else {
                    snippet_text.map(|snippet_text| (text.into_owned() + &snippet_text).into())
                }
            })
            .ok(),
        resolved_pattern => resolved_pattern
            .get_last_binding()
            .and_then(|binding| binding.text(&context.lang).ok()),
    };
    let message = message.as_deref().unwrap_or("(no message)");

    let severity = severity
        .as_ref()
        .and_then(|severity| severity.text(&state.files, &context.lang).ok())
        .and_then(|severity| Severity::from_str(severity.as_ref()).ok())
        .unwrap_or(Severity::Error);

    context.add_diagnostic(
        RuleDiagnostic::new(category!("plugin"), span, message).with_severity(severity),
    );

    Ok(span_node.clone())
}
