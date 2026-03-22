use crate::{AnalyzerPlugin, PluginDiagnostic, file_matches_includes};
use biome_analyze::{
    PluginActionData, PluginDiagnosticEntry, PluginEvalResult, PluginTargetLanguage, RuleDiagnostic,
};
use biome_console::markup;
use biome_css_syntax::{CssRoot, CssSyntaxNode};
use biome_diagnostics::{Applicability, Severity, category};
use biome_fs::FileSystem;
use biome_glob::NormalizedGlob;
use biome_grit_patterns::{
    BuiltInFunction, CompilePatternOptions, GritBinding, GritExecContext, GritPattern, GritQuery,
    GritQueryContext, GritQueryEffect, GritQueryState, GritResolvedPattern, GritTargetFile,
    GritTargetLanguage, compile_pattern_with_options,
};
use biome_js_syntax::{AnyJsRoot, JsSyntaxNode};
use biome_json_syntax::{JsonRoot, JsonSyntaxNode};
use biome_parser::{AnyParse, NodeParse};
use biome_rowan::{AnySyntaxNode, AstNode, RawSyntaxKind, SyntaxKind, TextRange};
use camino::{Utf8Path, Utf8PathBuf};
use grit_pattern_matcher::{binding::Binding, pattern::ResolvedPattern};
use grit_util::{AnalysisLogs, error::GritPatternError};
use std::{borrow::Cow, fmt::Debug, str::FromStr, sync::Arc};

/// Definition of an analyzer plugin backed by a GritQL query.
#[derive(Debug)]
pub struct AnalyzerGritPlugin {
    grit_query: GritQuery,

    /// Glob patterns that restrict which files this plugin runs on.
    /// `None` means the plugin runs on all files.
    /// `Some(&[])` (an empty list) means the plugin never runs on any file.
    includes: Option<Box<[NormalizedGlob]>>,
}

impl AnalyzerGritPlugin {
    pub fn load(
        fs: &dyn FileSystem,
        path: &Utf8Path,
        includes: Option<&[NormalizedGlob]>,
    ) -> Result<Self, PluginDiagnostic> {
        let source = fs.read_file_from_path(path)?;
        let options = CompilePatternOptions::default()
            .with_extra_built_ins(vec![
                BuiltInFunction::new(
                    "register_diagnostic",
                    &["span", "message", "severity", "fix_kind"],
                    Box::new(register_diagnostic),
                )
                .as_predicate(),
            ])
            .with_path(path);
        let grit_query = compile_pattern_with_options(&source, options)?;

        Ok(Self {
            grit_query,
            includes: includes.map(Into::into),
        })
    }
}

impl AnalyzerPlugin for AnalyzerGritPlugin {
    fn language(&self) -> PluginTargetLanguage {
        match &self.grit_query.language {
            GritTargetLanguage::JsTargetLanguage(_) => PluginTargetLanguage::JavaScript,
            GritTargetLanguage::CssTargetLanguage(_) => PluginTargetLanguage::Css,
            GritTargetLanguage::JsonTargetLanguage(_) => PluginTargetLanguage::Json,
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
            PluginTargetLanguage::Json => JsonRoot::KIND_SET
                .iter()
                .map(|kind| kind.to_raw())
                .collect(),
        }
    }

    fn applies_to_file(&self, path: &Utf8Path) -> bool {
        file_matches_includes(self.includes.as_deref(), path)
    }

    fn evaluate(&self, node: AnySyntaxNode, path: Arc<Utf8PathBuf>) -> PluginEvalResult {
        let name: &str = self.grit_query.name.as_deref().unwrap_or("anonymous");

        let (root, source_range, original_text) = match self.language() {
            PluginTargetLanguage::JavaScript => node
                .downcast_ref::<JsSyntaxNode>()
                .map(|node| {
                    let range = node.text_range_with_trivia();
                    let text = node.text_with_trivia().to_string();
                    (node.as_send(), range, text)
                })
                .unwrap(),
            PluginTargetLanguage::Css => node
                .downcast_ref::<CssSyntaxNode>()
                .map(|node| {
                    let range = node.text_range_with_trivia();
                    let text = node.text_with_trivia().to_string();
                    (node.as_send(), range, text)
                })
                .unwrap(),
            PluginTargetLanguage::Json => node
                .downcast_ref::<JsonSyntaxNode>()
                .map(|node| {
                    let range = node.text_range_with_trivia();
                    let text = node.text_with_trivia().to_string();
                    (node.as_send(), range, text)
                })
                .unwrap(),
        };

        let parse = AnyParse::Node(NodeParse::new(root.unwrap(), vec![]));
        let file = GritTargetFile { parse, path };

        match self.grit_query.execute_optimized(file) {
            Ok(result) => {
                // Log entries never consume actions.
                let log_entries = result.logs.iter().map(|log| PluginDiagnosticEntry {
                    diagnostic: RuleDiagnostic::new(
                        category!("plugin"),
                        log.range.map(from_grit_range),
                        markup!(<Emphasis>{name}</Emphasis>" logged: "<Info>{log.message}</Info>),
                    )
                    .verbose()
                    .subcategory(name.to_string()),
                    action: None,
                });

                // Convert rewrite effects to plugin actions.
                let mut actions: Vec<_> = result
                    .effects
                    .iter()
                    .filter_map(|effect| match effect {
                        GritQueryEffect::Rewrite(rewrite) => Some(PluginActionData {
                            source_range,
                            original_text: original_text.clone(),
                            rewritten_text: rewrite.rewritten.content.clone(),
                            message: format!("Rewrite suggested by plugin `{name}`"),
                            applicability: Applicability::MaybeIncorrect,
                        }),
                        _ => None,
                    })
                    .collect();

                // Pair each real diagnostic with its action by position.
                let mut action_iter = actions.drain(..);
                let diag_entries: Vec<_> = result
                    .diagnostics
                    .into_iter()
                    .map(|(diagnostic, applicability)| {
                        let mut action = action_iter.next();
                        if let Some(ref mut action) = action {
                            action.applicability = applicability;
                        }
                        PluginDiagnosticEntry {
                            diagnostic: diagnostic.subcategory(name.to_string()),
                            action,
                        }
                    })
                    .collect();

                let has_missing_span = diag_entries.iter().any(|e| e.diagnostic.span().is_none());

                let mut entries: Vec<_> = log_entries.chain(diag_entries).collect();

                if has_missing_span {
                    entries.push(PluginDiagnosticEntry {
                        diagnostic: RuleDiagnostic::new(
                            category!("plugin"),
                            None::<TextRange>,
                            markup!(
                                "Plugin "<Emphasis>{name}</Emphasis>" reported one or more diagnostics, "
                                "but it didn't specify a valid "<Emphasis>"span"</Emphasis>". "
                                "Diagnostics have been shown without context."
                            ),
                        ),
                        action: None,
                    });
                }

                PluginEvalResult { entries }
            }
            Err(error) => PluginEvalResult {
                entries: vec![PluginDiagnosticEntry {
                    diagnostic: RuleDiagnostic::new(
                        category!("plugin"),
                        None::<TextRange>,
                        markup!(<Emphasis>{name}</Emphasis>" errored: "<Error>{error.to_string()}</Error>),
                    ),
                    action: None,
                }],
            },
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

    let (span_node, message, severity, fix_kind) = match args.as_slice() {
        [Some(span), Some(message), severity, fix_kind] => (span, message, severity, fix_kind),
        _ => {
            return Err(GritPatternError::new(
                "register_diagnostic() takes 2 required arguments: span and message, and optional severity and fix_kind",
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

    let severity = match severity.as_ref() {
        None => Severity::Error,
        Some(severity) => {
            let text = severity
                .text(&state.files, &context.lang)
                .map_err(|e| GritPatternError::new(format!("failed to read severity: {e}")))?;
            Severity::from_str(text.as_ref()).map_err(|_| {
                GritPatternError::new(format!(
                    "invalid severity \"{text}\", expected \"hint\", \"info\", \"warn\", or \"error\""
                ))
            })?
        }
    };

    let applicability = match fix_kind.as_ref() {
        None => Applicability::MaybeIncorrect,
        Some(fix_kind) => {
            let text = fix_kind
                .text(&state.files, &context.lang)
                .map_err(|e| GritPatternError::new(format!("failed to read fix_kind: {e}")))?;
            match text.as_ref() {
                "safe" => Applicability::Always,
                "unsafe" => Applicability::MaybeIncorrect,
                other => {
                    return Err(GritPatternError::new(format!(
                        "invalid fix_kind \"{other}\", expected \"safe\" or \"unsafe\""
                    )));
                }
            }
        }
    };

    context.add_diagnostic(
        RuleDiagnostic::new(category!("plugin"), span, message).with_severity(severity),
        applicability,
    );

    Ok(span_node.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_fs::MemoryFileSystem;

    fn load_test_plugin(includes: Option<&[NormalizedGlob]>) -> AnalyzerGritPlugin {
        let fs = MemoryFileSystem::default();
        fs.insert("/test.grit".into(), r#"`hello`"#);
        AnalyzerGritPlugin::load(&fs, Utf8Path::new("/test.grit"), includes).unwrap()
    }

    #[test]
    fn applies_to_all_files_without_includes() {
        let plugin = load_test_plugin(None);
        assert!(plugin.applies_to_file(Utf8Path::new("src/main.ts")));
        assert!(plugin.applies_to_file(Utf8Path::new("test/foo.js")));
    }

    #[test]
    fn applies_to_matching_files_with_includes() {
        let globs: Vec<NormalizedGlob> = vec!["src/**/*.ts".parse().unwrap()];
        let plugin = load_test_plugin(Some(&globs));
        assert!(plugin.applies_to_file(Utf8Path::new("src/main.ts")));
        assert!(plugin.applies_to_file(Utf8Path::new("src/nested/file.ts")));
    }

    #[test]
    fn rejects_non_matching_files_with_includes() {
        let globs: Vec<NormalizedGlob> = vec!["src/**/*.ts".parse().unwrap()];
        let plugin = load_test_plugin(Some(&globs));
        assert!(!plugin.applies_to_file(Utf8Path::new("test/foo.ts")));
        assert!(!plugin.applies_to_file(Utf8Path::new("src/main.js")));
    }

    #[test]
    fn applies_with_negated_glob_exclusion() {
        let globs: Vec<NormalizedGlob> = vec![
            "src/**/*.ts".parse().unwrap(),
            "!**/*.test.ts".parse().unwrap(),
        ];
        let plugin = load_test_plugin(Some(&globs));
        assert!(plugin.applies_to_file(Utf8Path::new("src/main.ts")));
        assert!(!plugin.applies_to_file(Utf8Path::new("src/foo.test.ts")));
    }

    #[test]
    fn glob_does_not_match_absolute_paths_without_prefix() {
        let globs: Vec<NormalizedGlob> = vec!["src/**/*.ts".parse().unwrap()];
        let plugin = load_test_plugin(Some(&globs));
        // Relative paths match as expected
        assert!(plugin.applies_to_file(Utf8Path::new("src/main.ts")));
        // Absolute paths do NOT match a relative glob — this is expected behavior.
        // Users should use `**/src/**/*.ts` for absolute path matching.
        assert!(!plugin.applies_to_file(Utf8Path::new("/project/src/main.ts")));
    }

    #[test]
    fn empty_includes_matches_nothing() {
        let globs: Vec<NormalizedGlob> = vec![];
        let plugin = load_test_plugin(Some(&globs));
        assert!(!plugin.applies_to_file(Utf8Path::new("src/main.ts")));
        assert!(!plugin.applies_to_file(Utf8Path::new("any/file.js")));
    }
}
