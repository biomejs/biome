use super::{
    compilation_context::NodeCompilationContext, predicate_compiler::PredicateCompiler,
    PatternCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritPatternIfElse, GritPredicateIfElse};
use grit_pattern_matcher::pattern::{If, PrIf};

pub(crate) struct IfCompiler;

impl IfCompiler {
    pub(crate) fn from_node(
        node: &GritPatternIfElse,
        context: &mut NodeCompilationContext,
    ) -> Result<If<GritQueryContext>, CompileError> {
        let if_ = PredicateCompiler::from_node(&node.if_predicate()?, context)?;
        let then = PatternCompiler::from_maybe_curly_node(&node.then_pattern()?, context)?;
        let else_ = node
            .else_clause()
            .map(|node| PatternCompiler::from_maybe_curly_node(&node.else_pattern()?, context))
            .transpose()?;

        Ok(If::new(if_, then, else_))
    }
}

pub(crate) struct PrIfCompiler;

impl PrIfCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateIfElse,
        context: &mut NodeCompilationContext,
    ) -> Result<PrIf<GritQueryContext>, CompileError> {
        let if_ = PredicateCompiler::from_node(&node.if_predicate()?, context)?;
        let then = PredicateCompiler::from_node(&node.then_predicate()?, context)?;
        let else_ = node
            .else_clause()
            .map(|node| PredicateCompiler::from_node(&node.else_predicate()?, context))
            .transpose()?;

        Ok(PrIf::new(if_, then, else_))
    }
}
