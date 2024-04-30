use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternMaybe;
use grit_pattern_matcher::pattern::Maybe;

pub(crate) struct MaybeCompiler;

impl MaybeCompiler {
    pub(crate) fn from_node(
        node: &GritPatternMaybe,
        context: &mut NodeCompilationContext,
    ) -> Result<Maybe<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_maybe_curly_node(&node.pattern()?, context)?;

        Ok(Maybe::new(pattern))
    }
}
