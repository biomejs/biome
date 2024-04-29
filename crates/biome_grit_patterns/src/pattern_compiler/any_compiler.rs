use super::{compilation_context::CompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternAny;
use grit_pattern_matcher::pattern::Any;

pub(crate) struct AnyCompiler;

impl AnyCompiler {
    pub(crate) fn from_node(
        node: &GritPatternAny,
        context: &mut CompilationContext,
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
