use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternIncludes;
use grit_pattern_matcher::pattern::Includes;

pub(crate) struct IncludesCompiler;

impl IncludesCompiler {
    pub(crate) fn from_node(
        node: &GritPatternIncludes,
        context: &mut NodeCompilationContext,
    ) -> Result<Includes<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_maybe_curly_node(&node.includes()?, context)?;

        Ok(Includes::new(pattern))
    }
}
