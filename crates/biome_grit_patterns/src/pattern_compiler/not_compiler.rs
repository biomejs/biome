use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{diagnostics::CompilerDiagnostic, grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternNot;
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{Not, Pattern, PatternOrPredicate, Predicate};

pub(crate) struct NotCompiler;

impl NotCompiler {
    pub(crate) fn from_node(
        node: &GritPatternNot,
        context: &mut NodeCompilationContext,
    ) -> Result<Not<GritQueryContext>, CompileError> {
        let pattern = node.pattern()?;
        let pattern = PatternCompiler::from_node(&pattern, context)?;
        if pattern.iter().any(|p| {
            matches!(
                p,
                PatternOrPredicate::Pattern(Pattern::Rewrite(_))
                    | PatternOrPredicate::Predicate(Predicate::Rewrite(_))
            )
        }) {
            context.log(CompilerDiagnostic::new_warning(
                "Rewrites inside of a not will never be applied",
                node.syntax().text_range(),
            ));
        }
        Ok(Not::new(pattern))
    }
}
