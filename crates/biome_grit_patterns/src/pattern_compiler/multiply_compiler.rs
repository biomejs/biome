use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::GritMulOperation;
use grit_pattern_matcher::pattern::Multiply;

pub(crate) struct MultiplyCompiler;

impl MultiplyCompiler {
    pub(crate) fn from_node(
        node: &GritMulOperation,
        context: &mut NodeCompilationContext,
    ) -> Result<Multiply<GritQueryContext>, CompileError> {
        let left = PatternCompiler::from_node(&node.left()?, context)?;
        let right = PatternCompiler::from_node(&node.right()?, context)?;

        Ok(Multiply::new(left, right))
    }
}
