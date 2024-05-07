use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritLike;
use grit_pattern_matcher::pattern::{FloatConstant, Like, Pattern};

pub(crate) struct LikeCompiler;

impl LikeCompiler {
    pub(crate) fn from_node(
        node: &GritLike,
        context: &mut NodeCompilationContext,
    ) -> Result<Like<GritQueryContext>, CompileError> {
        let like = PatternCompiler::from_node(&node.example()?, context)?;
        let threshold = node
            .threshold()
            .map(|node| PatternCompiler::from_node(&node.threshold()?, context))
            .transpose()?
            .unwrap_or(Pattern::FloatConstant(FloatConstant::new(0.9)));

        Ok(Like::new(like, threshold))
    }
}
