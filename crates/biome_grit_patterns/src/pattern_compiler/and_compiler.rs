use super::{compilation_context::CompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternAnd;
use grit_pattern_matcher::pattern::And;

pub(crate) struct AndCompiler;

impl AndCompiler {
    pub(crate) fn from_node(
        node: &GritPatternAnd,
        context: &mut CompilationContext,
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
