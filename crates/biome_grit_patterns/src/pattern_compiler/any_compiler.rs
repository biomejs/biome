use super::{
    compilation_context::NodeCompilationContext, predicate_compiler::PredicateCompiler,
    PatternCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritPatternAny, GritPredicateAny};
use grit_pattern_matcher::pattern::{Any, PrAny};

pub(crate) struct AnyCompiler;

impl AnyCompiler {
    pub(crate) fn from_node(
        node: &GritPatternAny,
        context: &mut NodeCompilationContext,
    ) -> Result<Any<GritQueryContext>, CompileError> {
        let patterns = node
            .patterns()
            .into_iter()
            .map(|pattern| match pattern {
                Ok(pattern) => Ok(PatternCompiler::from_node(&pattern, context)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Any::new(patterns))
    }
}

pub(crate) struct PrAnyCompiler;

impl PrAnyCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateAny,
        context: &mut NodeCompilationContext,
    ) -> Result<PrAny<GritQueryContext>, CompileError> {
        let predicates = node
            .predicates()
            .into_iter()
            .map(|predicate| match predicate {
                Ok(predicate) => Ok(PredicateCompiler::from_node(&predicate, context)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PrAny::new(predicates))
    }
}
