use super::{
    compilation_context::NodeCompilationContext, predicate_compiler::PredicateCompiler,
    PatternCompiler,
};
use crate::{diagnostics::CompilerDiagnostic, grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritPatternNot, GritPredicateNot};
use biome_rowan::AstNode;
use grit_pattern_matcher::{
    context::StaticDefinitions,
    pattern::{Not, Pattern, PatternOrPredicate, PrNot, Predicate},
};

pub(crate) struct NotCompiler;

impl NotCompiler {
    pub(crate) fn from_node(
        node: &GritPatternNot,
        context: &mut NodeCompilationContext,
    ) -> Result<Not<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_node(&node.pattern()?, context)?;
        if pattern.iter(&StaticDefinitions::default()).any(|p| {
            matches!(
                p,
                PatternOrPredicate::Pattern(Pattern::Rewrite(_))
                    | PatternOrPredicate::Predicate(Predicate::Rewrite(_))
            )
        }) {
            context.log(CompilerDiagnostic::new_warning(
                "Rewrites inside of a not will never be applied",
                node.syntax().text_range_with_trivia(),
            ));
        }

        Ok(Not::new(pattern))
    }
}

pub(crate) struct PrNotCompiler;

impl PrNotCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateNot,
        context: &mut NodeCompilationContext,
    ) -> Result<PrNot<GritQueryContext>, CompileError> {
        let predicate = PredicateCompiler::from_node(&node.predicate()?, context)?;
        if predicate.iter(&StaticDefinitions::default()).any(|p| {
            matches!(
                p,
                PatternOrPredicate::Pattern(Pattern::Rewrite(_))
                    | PatternOrPredicate::Predicate(Predicate::Rewrite(_))
            )
        }) {
            context.log(CompilerDiagnostic::new_warning(
                "Rewrites inside of a not will never be applied",
                node.syntax().text_range_with_trivia(),
            ));
        }

        Ok(PrNot::new(predicate))
    }
}
