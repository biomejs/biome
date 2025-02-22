use biome_analyze::RuleDiagnostic;
use biome_console::markup;
use biome_diagnostics::category;
use biome_fs::FileSystem;
use biome_grit_patterns::{
    compile_pattern_with_options, BuiltInFunction, CompilePatternOptions, GritBinding,
    GritExecContext, GritPattern, GritQuery, GritQueryContext, GritQueryState, GritResolvedPattern,
    GritTargetFile,
};
use biome_parser::AnyParse;
use biome_rowan::TextRange;
use camino::{Utf8Path, Utf8PathBuf};
use grit_pattern_matcher::{binding::Binding, pattern::ResolvedPattern};
use grit_util::{error::GritPatternError, AnalysisLogs};
use std::{borrow::Cow, fmt::Debug};

use crate::{AnalyzerPlugin, PluginDiagnostic};

/// Definition of an analyzer plugin.
#[derive(Debug)]
pub struct AnalyzerGritPlugin {
    grit_query: GritQuery,
}

impl AnalyzerGritPlugin {
    pub fn load(fs: &dyn FileSystem, path: &Utf8Path) -> Result<Self, PluginDiagnostic> {
        let source = fs.read_file_from_path(path)?;
        let options = CompilePatternOptions::default()
            .with_extra_built_ins(vec![BuiltInFunction::new(
                "register_diagnostic",
                &[
                    "span",
                    "message",
                    "fixer_description",
                    "category",
                    "applicability",
                ],
                Box::new(register_diagnostic),
            )
            .as_predicate()])
            .with_path(path);
        let grit_query = compile_pattern_with_options(&source, options)?;

        Ok(Self { grit_query })
    }
}

impl AnalyzerPlugin for AnalyzerGritPlugin {
    fn evaluate(&self, root: AnyParse, path: Utf8PathBuf) -> Vec<RuleDiagnostic> {
        let name: &str = self.grit_query.name.as_deref().unwrap_or("anonymous");

        let file = GritTargetFile { parse: root, path };
        match self.grit_query.execute(file) {
            Ok(result) => result
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
                .collect(),
            Err(error) => vec![RuleDiagnostic::new(
                category!("plugin"),
                None::<TextRange>,
                markup!(<Emphasis>{name}</Emphasis>" errored: "<Error>{error.to_string()}</Error>),
            )],
        }
    }

    fn supports_css(&self) -> bool {
        self.grit_query.supports_css()
    }

    fn supports_js(&self) -> bool {
        self.grit_query.supports_js()
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

    let (span_node, message, _fixer_description, _category, _applicability) = match args.as_slice() {
        [Some(span), Some(message), None, None, None] => (span, message, None, None, None),
        [Some(span), Some(message), Some(fixer_description), Some(category), Some(applicability)] => (span, message, Some(fixer_description), Some(category), Some(applicability)),
        // TODO: Do we want to make `category` and `applicability` optional, even for rules with a fixer?
        _ => return Err(GritPatternError::new(
            "register_diagnostic() takes 2 or 5 arguments: span and message, and optional fixer_description, category and applicability",
        )),
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

    context.add_diagnostic(RuleDiagnostic::new(category!("plugin"), span, message));

    Ok(span_node.clone())
}
