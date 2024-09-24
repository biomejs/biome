use crate::{rule::RuleAdvice, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{
    adapters::{IoError, ResolveError},
    category, Diagnostic, DiagnosticTags,
};
use biome_grit_patterns::{CompileError, GritQuery, GritQueryResult, GritTargetFile};
use biome_rowan::{AstNode, Language, TextRange};
use std::{fmt::Debug, path::PathBuf, rc::Rc};

/// Definition of an analyzer plugin.
#[derive(Clone, Debug)]
pub struct AnalyzerPlugin {
    grit_query: Rc<GritQuery>,
}

impl From<GritQuery> for AnalyzerPlugin {
    fn from(grit_query: GritQuery) -> Self {
        Self {
            grit_query: Rc::new(grit_query),
        }
    }
}

impl AnalyzerPlugin {
    pub fn evaluate<L: Language + 'static>(
        &self,
        root: &L::Root,
        path: PathBuf,
    ) -> Vec<RuleDiagnostic> {
        let name: &str = self.grit_query.name.as_deref().unwrap_or("anonymous");

        let file = GritTargetFile {
            parse: root.syntax().as_send().expect("not a root node").into(),
            path,
        };
        match self.grit_query.execute(file) {
            Ok((results, logs)) => results
                .into_iter()
                .filter_map(|result| match result {
                    GritQueryResult::Match(match_) => Some(match_),
                    GritQueryResult::Rewrite(_) | GritQueryResult::CreateFile(_) => None,
                })
                .map(|match_| RuleDiagnostic {
                    category: category!("plugin"),
                    span: match_
                        .ranges
                        .into_iter()
                        .next()
                        .map(RangeExt::to_text_range),
                    // TODO: Plugin should be able to provide its own message
                    message: markup!(<Emphasis>{name}</Emphasis>" matched").into(),
                    tags: DiagnosticTags::empty(),
                    rule_advice: RuleAdvice::default(),
                })
                .chain(logs.iter().map(|log| RuleDiagnostic {
                    category: category!("plugin"),
                    span: log.range.map(RangeExt::to_text_range),
                    message: markup!(<Emphasis>{name}</Emphasis>" logged: "<Info>{log.message}</Info>)
                        .into(),
                    tags: DiagnosticTags::VERBOSE,
                    rule_advice: RuleAdvice::default(),
                }))
                .collect(),
            Err(error) => vec![RuleDiagnostic {
                category: category!("plugin"),
                span: None,
                message: markup!(<Emphasis>{name}</Emphasis>" errored: "<Error>{error.to_string()}</Error>)
                    .into(),
                tags: DiagnosticTags::empty(),
                rule_advice: RuleAdvice::default(),
            }],
        }
    }
}

#[derive(Debug, Diagnostic)]
pub enum PluginError {
    Compile(CompileError),
    Io(IoError),
    Resolve(ResolveError),
}

impl From<CompileError> for PluginError {
    fn from(error: CompileError) -> Self {
        Self::Compile(error)
    }
}

impl From<std::io::Error> for PluginError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.into())
    }
}

impl From<oxc_resolver::ResolveError> for PluginError {
    fn from(error: oxc_resolver::ResolveError) -> Self {
        Self::Resolve(error.into())
    }
}

trait RangeExt {
    fn to_text_range(self) -> TextRange;
}

impl RangeExt for grit_util::Range {
    fn to_text_range(self) -> TextRange {
        TextRange::new(self.start_byte.into(), self.end_byte.into())
    }
}
