use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::GritWithin;
use grit_pattern_matcher::pattern::Within;

pub(crate) struct WithinCompiler;

impl WithinCompiler {
    pub(crate) fn from_node(
        node: &GritWithin,
        context: &mut NodeCompilationContext,
    ) -> Result<Within<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_maybe_curly_node(&node.pattern()?, context)?;
        let until = node
            .until_clause()
            .map(|clause| PatternCompiler::from_node(&clause.until()?, context))
            .transpose()?;

        Ok(Within::new(pattern, until))
    }
}
