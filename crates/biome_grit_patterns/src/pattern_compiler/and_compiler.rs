use super::{
    compilation_context::NodeCompilationContext, predicate_compiler::PredicateCompiler,
    PatternCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritPatternAnd, GritPredicateAnd};
use grit_pattern_matcher::pattern::{And, PrAnd};

pub(crate) struct AndCompiler;

impl AndCompiler {
    pub(crate) fn from_node(
        node: &GritPatternAnd,
        context: &mut NodeCompilationContext,
    ) -> Result<And<GritQueryContext>, CompileError> {
        let patterns = node
            .patterns()
            .into_iter()
            .map(|pattern| match pattern {
                Ok(pattern) => Ok(PatternCompiler::from_node(&pattern, context)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(And::new(patterns))
    }
}

pub(crate) struct PrAndCompiler;

impl PrAndCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateAnd,
        context: &mut NodeCompilationContext,
    ) -> Result<PrAnd<GritQueryContext>, CompileError> {
        let predicates = node
            .predicates()
            .into_iter()
            .map(|predicate| match predicate {
                Ok(predicate) => Ok(PredicateCompiler::from_node(&predicate, context)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PrAnd::new(predicates))
    }
}
