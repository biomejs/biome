use super::{compilation_context::CompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternOr;
use grit_pattern_matcher::pattern::Or;

pub(crate) struct OrCompiler;

impl OrCompiler {
    pub(crate) fn from_node(
        node: &GritPatternOr,
        context: &mut CompilationContext,
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
