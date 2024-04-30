use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
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
