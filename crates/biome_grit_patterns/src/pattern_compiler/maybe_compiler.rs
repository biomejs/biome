use super::{
    compilation_context::NodeCompilationContext, predicate_compiler::PredicateCompiler,
    PatternCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritPatternMaybe, GritPredicateMaybe};
use grit_pattern_matcher::pattern::{Maybe, PrMaybe};

pub(crate) struct MaybeCompiler;

impl MaybeCompiler {
    pub(crate) fn from_node(
        node: &GritPatternMaybe,
        context: &mut NodeCompilationContext,
    ) -> Result<Maybe<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_maybe_curly_node(&node.pattern()?, context)?;

        Ok(Maybe::new(pattern))
    }
}

pub(crate) struct PrMaybeCompiler;

impl PrMaybeCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateMaybe,
        context: &mut NodeCompilationContext,
    ) -> Result<PrMaybe<GritQueryContext>, CompileError> {
        let predicate = PredicateCompiler::from_node(&node.predicate()?, context)?;

        Ok(PrMaybe::new(predicate))
    }
}
