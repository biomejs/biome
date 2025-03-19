use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::GritEvery;
use grit_pattern_matcher::pattern::Every;

pub(crate) struct EveryCompiler;

impl EveryCompiler {
    pub(crate) fn from_node(
        node: &GritEvery,
        context: &mut NodeCompilationContext,
    ) -> Result<Every<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_maybe_curly_node(&node.pattern()?, context)?;

        Ok(Every::new(pattern))
    }
}
