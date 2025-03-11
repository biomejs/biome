use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::GritSome;
use grit_pattern_matcher::pattern::Some;

pub(crate) struct SomeCompiler;

impl SomeCompiler {
    pub(crate) fn from_node(
        node: &GritSome,
        context: &mut NodeCompilationContext,
    ) -> Result<Some<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_maybe_curly_node(&node.pattern()?, context)?;

        Ok(Some::new(pattern))
    }
}
