use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternContains;
use grit_pattern_matcher::pattern::Contains;

pub(crate) struct ContainsCompiler;

impl ContainsCompiler {
    pub(crate) fn from_node(
        node: &GritPatternContains,
        context: &mut NodeCompilationContext,
    ) -> Result<Contains<GritQueryContext>, CompileError> {
        let contains = PatternCompiler::from_maybe_curly_node(&node.contains()?, context)?;
        let until = node
            .until_clause()
            .map(|node| PatternCompiler::from_node(&node.until()?, context))
            .transpose()?;

        Ok(Contains::new(contains, until))
    }
}
