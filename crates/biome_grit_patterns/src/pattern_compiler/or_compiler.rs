use super::{
    compilation_context::NodeCompilationContext, predicate_compiler::PredicateCompiler,
    PatternCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritPatternOr, GritPredicateOr};
use grit_pattern_matcher::pattern::{Or, PrOr};

pub(crate) struct OrCompiler;

impl OrCompiler {
    pub(crate) fn from_node(
        node: &GritPatternOr,
        context: &mut NodeCompilationContext,
    ) -> Result<Or<GritQueryContext>, CompileError> {
        let patterns = node
            .patterns()
            .into_iter()
            .map(|pattern| match pattern {
                Ok(pattern) => Ok(PatternCompiler::from_node(&pattern, context)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Or::new(patterns))
    }
}

pub(crate) struct PrOrCompiler;

impl PrOrCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateOr,
        context: &mut NodeCompilationContext,
    ) -> Result<PrOr<GritQueryContext>, CompileError> {
        let predicates = node
            .predicates()
            .into_iter()
            .map(|predicate| match predicate {
                Ok(predicate) => Ok(PredicateCompiler::from_node(&predicate, context)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PrOr::new(predicates))
    }
}
